use macroquad::prelude::*;

use crate::arena::Arena;
use crate::boss::BOSS_SPECIAL_HP_THRESHOLDS;
use crate::constants::{
    BACKGROUND, GAME_OVER_BANNER_DURATION, HEIGHT, LOST_BANNER_DURATION, MAX_FRAME_DT,
    RESET_BANNER_DURATION, SHAKE_TRAUMA_PER_HIT, WIDTH,
};
use crate::dev_ui;
use crate::gfx::{Post, Shaders, Shake};
use crate::input::Input;
use crate::level_window::{LevelUpOption, LevelWindow, generate_placeholder_options};
use crate::state::GameState;
use crate::ui::Ui;

// things that the game events can emit
// like the player getting hit
pub enum GameEvent {
    PlayerHit,
    // boss damaged hit per frame
    BossHit { damage: i32 },
    // player set off a bomb at this position; clears nearby hazards
    BombDetonated { position: Vec2 },

    // pushed when (admin) resets the game state, to help render a
    // text to visually indicate the rest
    GameReset,

    // pushed when the boss finishes its death animation
    GameOver,

    // boss HP threshold crossed or tab pressed; carries the three upgrade
    // options to present. the world spawns a LevelWindow from this event
    LevelUp { options: [LevelUpOption; 3] },
}

// whether the simulation is advancing or frozen
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WorldState {
    Running,
    Paused,
}

// owns the per-frame plumbing (camera, timing, input) and the top-level pieces
// (arena, stage, ui, state), and orchestrates the frame.
pub struct World {
    last_time: f64,
    dt: f32,
    // camera maps logical rect (which can be affected by screen dpi) onto physical screen
    // drawing uses all logical space rect coordinates and camera converts
    camera: Camera2D,
    arena: Arena,
    state: GameState,
    ui: Ui,
    // GPU materials, loaded once and shared across the scene
    shaders: Shaders,
    // full-screen post-process pipeline (dormant until a pass is enabled)
    post: Post,
    // camera screen shake, fed trauma when the player gets hit
    shake: Shake,
    // seconds remaining on the centered "Reset" banner
    reset_banner: f32,
    // seconds remaining on the centered "Lost" banner
    lost_banner: f32,
    // seconds remaining on the centered "Game Over" banner
    game_over_banner: f32,
    // egui debug window, toggled with spacebar
    dev_ui: bool,
    // paused freezes the simulation; rendering keeps going
    world_state: WorldState,
    // modal upgrade window, if one is currently being shown. created on-demand
    // from a GameEvent::LevelUp; absent otherwise
    level_window: Option<LevelWindow>,
    // which boss HP-fraction thresholds have already triggered a level-up this
    // fight, so each one only fires once. cleared on game reset
    level_thresholds_opened: Vec<bool>,

    // drained every frame
    events: Vec<GameEvent>,
}

impl World {
    pub fn new() -> Self {
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT));
        // flip it upside down so (0, 0) is top left
        // and (WIDTH, HEIGHT) is bottom right
        camera.zoom.y = -camera.zoom.y;

        World {
            last_time: get_time(),
            dt: 0.0,
            camera,
            arena: Arena::new(),
            state: GameState::new(),
            ui: Ui::new(),
            shaders: Shaders::new(),
            post: Post::new(),
            shake: Shake::new(),
            reset_banner: 0.0,
            lost_banner: 0.0,
            game_over_banner: 0.0,
            dev_ui: false,
            world_state: WorldState::Running,
            level_window: None,
            // to track that the level up window is only shown once per threshold
            level_thresholds_opened: vec![false; BOSS_SPECIAL_HP_THRESHOLDS.len()],
            events: Vec::new(),
        }
    }

    // refresh dt from the time elapsed since the previous frame, clamped so a
    // stutter frame can't teleport fast projectiles through the player
    fn compute_dt(&mut self) {
        let now = get_time();
        let raw = (now - self.last_time) as f32;
        self.dt = raw.min(MAX_FRAME_DT);
        self.last_time = now;
    }

    // step per frame and gets next frame until window closes
    pub async fn run(mut self) {
        loop {
            self.update();
            self.draw();
            next_frame().await;
        }
    }

    fn update(&mut self) {
        // always refresh dt, even while paused: skipping it would leave
        // last_time stale and feed the whole pause duration as one giant dt
        // into the first running frame
        self.compute_dt();

        // gather input here since World owns the camera (mouse -> world)
        let input = Input::gather(&self.camera);

        // if a level-up window is active, it gets exclusive control;
        // the game stays frozen until the player picks a card
        if let Some(window) = self.level_window.as_mut() {
            if let Some(index) = window.update(self.dt, input.screen_mouse, input.primary_pressed) {
                // add the selected modifier to the player's recipe
                let modifier = window.selected_modifier(index);
                self.arena.player_mut().projectile_recipe.add_modifier(modifier);
                self.level_window = None;
            }
            return;
        }

        if input.escape_pressed {
            self.world_state = match self.world_state {
                WorldState::Running => WorldState::Paused,
                WorldState::Paused => WorldState::Running,
            };
        }

        // dev ui is a debug overlay; keep it usable while paused
        if input.space_pressed {
            self.dev_ui = !self.dev_ui;
        }

        // paused freezes everything below: shake, banner countdowns, reset,
        // the simulation, and the event drain
        if self.world_state == WorldState::Paused {
            return;
        }

        // decay/advance the shake before applying this frame's events: a fresh
        // hit then renders at full trauma. decaying afterwards would clip the
        // opening kick by one frame's worth of decay.
        self.shake.update(self.dt);

        // for any banners, count them down before applying events
        self.reset_banner = (self.reset_banner - self.dt).max(0.0);
        self.lost_banner = (self.lost_banner - self.dt).max(0.0);
        self.game_over_banner = (self.game_over_banner - self.dt).max(0.0);

        // hotkey to force a level-up window for testing
        if input.tab_pressed && self.level_window.is_none() {
            self.events.push(GameEvent::LevelUp {
                options: generate_placeholder_options(),
            });
        }

        if input.tilde_pressed {
            self.reset_game();
            self.events.push(GameEvent::GameReset);
        }

        self.arena
            .update(self.dt, &input, &mut self.state, &mut self.events);

        // drain and process all pending events
        self.process_events();

        // check if any boss HP thresholds were crossed this frame;
        // the resulting LevelUp event opens next frame
        self.check_level_thresholds();
    }

    fn process_events(&mut self) {
        let mut events = std::mem::take(&mut self.events);
        for event in events.drain(..) {
            match event {
                GameEvent::PlayerHit => self.handle_player_hit(),
                GameEvent::BossHit { damage } => {
                    self.arena.damage_boss(damage);
                }
                GameEvent::BombDetonated { position } => {
                    self.state.bombs = self.state.bombs.saturating_sub(1);
                    self.arena.detonate_bomb(position);
                }
                GameEvent::GameReset => {
                    self.reset_banner = RESET_BANNER_DURATION;
                }
                GameEvent::GameOver => {
                    self.game_over_banner = GAME_OVER_BANNER_DURATION;
                }
                GameEvent::LevelUp { options } => {
                    self.level_window = Some(LevelWindow::new(options));
                }
            }
        }
        self.events = events;
    }

    fn handle_player_hit(&mut self) {
        self.state.lives = self.state.lives.saturating_sub(1);
        self.arena.player_mut().register_hit();
        self.shake.add_trauma(SHAKE_TRAUMA_PER_HIT);
        // reset game when health is 0
        if self.state.lives == 0 {
            self.reset_game();
            self.lost_banner = LOST_BANNER_DURATION;
        }
    }

    // drop any leftover level threshold states from the previous fight
    fn reset_level_state(&mut self) {
        self.level_window = None;
        self.level_thresholds_opened.fill(false);
    }

    // reinitialise arena + game state and clear level-up tracking
    fn reset_game(&mut self) {
        self.arena = Arena::new();
        self.state = GameState::new();
        self.reset_level_state();
    }

    // fire a LevelUp event if the boss HP just crossed a threshold
    fn check_level_thresholds(&mut self) {
        if self.level_window.is_some() {
            return;
        }
        let (current, total) = self.arena.boss_health();
        if current <= 0 || total <= 0 {
            return;
        }
        let frac = current as f32 / total as f32;
        for (i, &t) in BOSS_SPECIAL_HP_THRESHOLDS.iter().enumerate() {
            if !self.level_thresholds_opened[i] && frac <= t {
                self.level_thresholds_opened[i] = true;
                self.events.push(GameEvent::LevelUp {
                    options: generate_placeholder_options(),
                });
                // only one threshold can fire per frame
                break;
            }
        }
    }

    fn draw(&self) {
        // update the camera only if there is some shake effect
        let shaken = self
            .shake
            .is_active()
            .then(|| self.shake.apply(&self.camera));
        let camera = shaken.as_ref().unwrap_or(&self.camera);

        // if post processing pipeline is active, then pass it through the
        // offscreen rendering, otherwise go straight to screen
        if self.post.active() {
            self.post.begin(camera);
            self.arena.draw(&self.state, &self.shaders);
            self.post.present(&self.shaders);
        } else {
            set_camera(camera);
            clear_background(BACKGROUND);
            self.arena.draw(&self.state, &self.shaders);
        }

        // UI + egui always draw to the screen on top, unaffected by post
        self.ui.draw(
            &self.state,
            self.arena.bounds(),
            self.arena.player_damage(),
            self.arena.boss_health(),
            self.arena.boss_displayed_health(),
            self.arena.boss_invulnerable(),
            self.reset_banner,
            self.lost_banner,
            self.game_over_banner,
            self.world_state == WorldState::Paused,
        );

        // level-up window draws last so it covers the arena, the hud, and the
        // dev ui. it also resets to the default camera in case egui left its
        // own pipeline active
        if let Some(window) = &self.level_window {
            window.draw();
        }

        // always render dev ui on top of everything else
        if self.dev_ui {
            dev_ui::draw(&self.state, &self.arena);
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World::new()
    }
}
