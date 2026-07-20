use macroquad::prelude::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 800.0;

// longest simulation step a single frame may take, in seconds. stutter frames
// (window drags, os hiccups) would otherwise teleport fast bullets far enough
// to skip past the player in one step; clamping trades that teleport for a
// brief slow-motion instead
pub const MAX_FRAME_DT: f32 = 1.0 / 60.0;

// color scheme
pub const BACKGROUND: Color = Color::new(0.06, 0.07, 0.11, 1.0); // near-black navy
pub const ARENA_BORDER_COLOR: Color = Color::new(0.42, 0.44, 0.72, 1.0); // soft indigo frame

// player colors
pub const PLAYER_COLOR: Color = Color::new(0.25, 0.92, 0.85, 1.0); // cyan
pub const PLAYER_PHASING_COLOR: Color = Color::new(0.85, 1.0, 1.0, 1.0); // pale ghost cyan
pub const PLAYER_PROJECTILE_COLOR: Color = Color::new(0.55, 0.95, 1.0, 1.0); // light cyan
pub const PLAYER_TRAIL_COLOR: Color = Color::new(0.45, 0.85, 0.85, 1.0); // dim cyan

// boss colors
pub const BOSS_COLOR: Color = Color::new(0.96, 0.28, 0.55, 1.0); // hot magenta-pink
pub const BOSS_PROJECTILE_COLOR: Color = Color::new(1.0, 0.45, 0.30, 1.0); // warm coral
pub const BOSS_SPECIAL_PROJECTILE_COLOR: Color = Color::new(0.58, 0.24, 0.16, 1.0); // burnt coral

// hud colors
pub const UI_TEXT_COLOR: Color = Color::new(0.90, 0.92, 0.96, 1.0); // soft off-white

// startup window colors
pub const STARTUP_OVERLAY_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.75);
pub const STARTUP_WINDOW_BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const STARTUP_PRIMARY_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const STARTUP_MUTED_TEXT_COLOR: Color = Color::new(0.75, 0.75, 0.78, 1.0);
pub const STARTUP_DIVIDER_COLOR: Color = Color::new(0.35, 0.35, 0.40, 1.0);
pub const STARTUP_ACTION_TEXT_COLOR: Color = Color::new(0.9, 0.7, 0.3, 1.0);
pub const STARTUP_BUTTON_BG_COLOR: Color = Color::new(0.08, 0.08, 0.10, 1.0);
pub const STARTUP_BUTTON_HOVER_BG_COLOR: Color = Color::new(0.18, 0.18, 0.22, 1.0);
pub const STARTUP_BUTTON_BORDER_COLOR: Color = Color::new(0.55, 0.55, 0.60, 1.0);

// gap from left window edge to arena border
pub const ARENA_MARGIN_WIDTH: f32 = 80.0;
// gap from top/bottom window edge to arena border
pub const ARENA_MARGIN_HEIGHT: f32 = 40.0;
pub const ARENA_BORDER_THICKNESS: f32 = 2.0;

pub const PLAYER_CIRCLE_RADIUS: f32 = 6.0;
pub const PLAYER_SPEED: f32 = 200.0;
// seconds between player shots (the player fires continuously)
pub const PLAYER_FIRE_INTERVAL: f32 = 0.20;
pub const PLAYER_DEV_CHEAT_FIRE_INTERVAL: f32 = 0.05;
pub const PLAYER_DEV_BOMBS: u32 = 9999;
// upward travel speed of player bullets in pixels per second
pub const PLAYER_PROJECTILE_SPEED: f32 = 450.0;

pub const BOSS_WIDTH: f32 = 150.0;
pub const BOSS_HEIGHT: f32 = 150.0;
pub const BOSS_HEALTH: i32 = 5000;

// clockwise spin speed in radians per second
// used when boss is idle (the steady speed it settles to)
pub const BOSS_IDLE_ROTATION_SPEED: f32 = 0.8;
// fastest speed
pub const BOSS_SPINUP_PEAK_SPEED: f32 = 12.0;
// slow -> fast for this many seconds
pub const BOSS_SPINUP_RAMP_UP: f32 = 0.8;
// hold atp eak for how long
pub const BOSS_SPINUP_HOLD: f32 = 1.0;
// easing down for this many seconds
pub const BOSS_SPINUP_RAMP_DOWN: f32 = 1.5;
// total duration
pub const BOSS_SPINUP_DURATION: f32 =
    BOSS_SPINUP_RAMP_UP + BOSS_SPINUP_HOLD + BOSS_SPINUP_RAMP_DOWN;

// death burst: a ring that expands from the boss center out past the arena
// edges, deleting every projectile it sweeps over before the boss falls.
// radius growth in px/s
pub const BOSS_DEATH_BURST_SPEED: f32 = 1600.0;
pub const BOSS_DEATH_BURST_COLOR: Color = Color::new(0.85, 0.95, 1.0, 1.0); // bright pale cyan
pub const BOSS_DEATH_BURST_THICKNESS: f32 = 5.0;

// death animation: the boss spins up and falls off the bottom of the screen.
// long hold so it keeps spinning at peak speed the whole way down
pub const BOSS_DEATH_SPIN_HOLD: f32 = 5.0;
// downward acceleration of the fall (px/s^2)
pub const BOSS_DEATH_GRAVITY: f32 = 1200.0;
// initial downward speed when the death drop begins (px/s)
pub const BOSS_DEATH_INITIAL_DROP_SPEED: f32 = 30.0;

// seconds between shots while the boss is idle
pub const BOSS_FIRE_INTERVAL: f32 = 0.5;
// number of projectiles in each all-directions burst
pub const BOSS_PROJECTILE_COUNT: usize = 15;
pub const BOSS_AIM_STEP: f32 = 0.12;
pub const BOSS_AIM_STEPS: i32 = 3;

// seconds between volleys in the clustered special move
pub const BOSS_SPECIAL_FIRE_INTERVAL: f32 = 0.4;
pub const BOSS_SPECIAL_SPINUP_HOLD: f32 = 2.0;
// total duration of the special move
pub const BOSS_SPECIAL_DURATION: f32 =
    BOSS_SPINUP_RAMP_UP + BOSS_SPECIAL_SPINUP_HOLD + BOSS_SPINUP_RAMP_DOWN;
pub const BOSS_CLUSTER_SHOTS: usize = 6;
pub const BOSS_CLUSTER_COUNT: usize = 5;
pub const BOSS_CLUSTER_INTRA_GAP: f32 = 0.11;
pub const BOSS_SPECIAL_SWEEP_STEP: f32 = 0.12;
// these projectiles should move faster so its harder
pub const BOSS_SPECIAL_PROJECTILE_SPEED: f32 = 150.0;
pub const BOSS_TRANSITION_75_DURATION: f32 = 4.0;

pub const PROJECTILE_RADIUS: f32 = 6.0;
// travel speed in pixels per second
pub const PROJECTILE_SPEED: f32 = 120.0;

// the boss fires a beam volley on this interval. the telegraph + active window is
// ~4.5s (see BEAM_STARTUP_DURATION + BEAM_ACTIVE_DURATION), so this stays comfortably
// longer to keep the beam attack infrequent
pub const BOSS_BEAM_INTERVAL: f32 = 6.0;

// how far past arena bounds the frame mask is drawn to hide the beam overshoot
pub const FRAME_MASK_PAD: f32 = 1000.0;
// persistent sweeping beam fired by the boss
pub const BEAM_WIDTH: f32 = 16.0;
// beams are drawn overshooting the arena by this much so their flat end caps never
// leave a triangular gap at the edges; the overflow is hidden by the arena frame mask
pub const BEAM_EDGE_OVERSHOOT: f32 = 50.0;
// beam colour when active
pub const BEAM_COLOR: Color = Color::new(1.0, 0.25, 0.35, 0.95);
// the thickness to show when the beam is inactive
pub const BEAM_BORDER_THICKNESS: f32 = 2.0;
// beam time taken to become active
pub const BEAM_STARTUP_DURATION: f32 = 1.5;
pub const BEAM_ACTIVE_DURATION: f32 = 3.0;

// invulnerability window granted to the player after any hit
pub const HIT_INVULN_DURATION: f32 = 0.5;
// invulnerability window granted after choosing an upgrade from the level window
pub const LEVEL_WINDOW_INVULN_DURATION: f32 = 0.5;

// screen shake (trauma-based): each hit bumps trauma, which decays every second.
// the visible shake scales with trauma^2 so small bumps stay subtle.
pub const SHAKE_TRAUMA_PER_HIT: f32 = 0.6;
// trauma lost per second; 0.6 trauma fully decays in 0.6 / 1.2 = 0.5s
pub const SHAKE_DECAY: f32 = 1.2;
// peak positional offset in logical units at full trauma
pub const SHAKE_MAX_OFFSET: f32 = 16.0;
// peak rotational kick in degrees at full trauma (set to 0.0 to disable tilt)
pub const SHAKE_MAX_ANGLE: f32 = 2.5;
// how fast the shake noise oscillates, in noise samples per second.
pub const SHAKE_FREQUENCY: f32 = 15.0;
pub const SHAKE_MIN_TRAUMA: f32 = 0.01;

pub const PHASE_DISTANCE: f32 = 150.0;
pub const PHASE_DURATION: f32 = 0.20;
pub const PHASE_MIN_OPACITY: f32 = 0.1;
pub const PHASE_TRAIL_LENGTH: usize = 3; // ghost circles drawn behind the phase
pub const PHASE_GHOST_OPACITY: f32 = 0.2; // opacity of the freshest ghost

// initial game state
pub const STARTING_LIVES: u32 = 3;
pub const STARTING_BOMBS: u32 = 3;

// boss health bar (top-of-screen hud)
pub const HEALTH_BAR_HEIGHT: f32 = 28.0;
// gap from the top window edge down to the bar
pub const HEALTH_BAR_TOP_MARGIN: f32 = 8.0;
// empty track colour
pub const HEALTH_BAR_BG_COLOR: Color = Color::new(0.15, 0.16, 0.22, 0.9);
// current health colour
pub const HEALTH_BAR_FILL_COLOR: Color = Color::new(0.90, 0.20, 0.25, 1.0);
// current health colour when the boss is invulnerable (special move / dying / dead)
pub const HEALTH_BAR_INVULN_FILL_COLOR: Color = Color::new(0.30, 0.55, 0.95, 1.0);
// thin vertical strip drawn over the bar at each boss special-move threshold
pub const HEALTH_BAR_MARKER_COLOR: Color = Color::new(0.95, 0.96, 1.0, 1.0);
pub const HEALTH_BAR_MARKER_THICKNESS: f32 = 2.0;
// animated chip colour
pub const HEALTH_BAR_CHIP_COLOR: Color = Color::new(0.98, 0.90, 0.65, 1.0);
pub const HEALTH_BAR_DROP_SPEED: f32 = 9.0;

// bomb: clears nearby enemy hazards for a short window when detonated
// clear radius around the player when the bomb goes off
pub const BOMB_RADIUS: f32 = 200.0;
// how long the bomb stays active (clearing + visible), in seconds.
// time-based so it's framerate independent (~10 frames at 60fps)
pub const BOMB_DURATION: f32 = 0.16;
// the ring drawn while the bomb is active
pub const BOMB_BORDER_COLOR: Color = Color::new(0.85, 0.95, 1.0, 1.0); // bright pale cyan
pub const BOMB_BORDER_THICKNESS: f32 = 3.0;

pub const BANNER_FONT_SIZE: f32 = 96.0;

// transient "Reset" banner shown centered on screen after an admin reset
pub const RESET_BANNER_DURATION: f32 = 0.75;
// lost banner
pub const LOST_BANNER_DURATION: f32 = 0.75;
// shown once the boss finishes its death animation
pub const GAME_OVER_BANNER_DURATION: f32 = 0.75;

// modifiers
pub const HOMING_TURN_SPEED: f32 = 4.0;
pub const LIGHTNING_DAMAGE_MULTIPLIER: f32 = 0.3;
pub const LIGHTNING_EFFECT_DURATION: f32 = 0.12; // Duration of chain lightning visual arc in seconds
// homing turns the projectiles purple
pub const HOMING_PROJECTILE_COLOR: Color = Color::new(0.45, 0.15, 0.70, 1.0);
// green ish colour
pub const BOUNCING_PROJECTILE_COLOR: Color = Color::new(0.20, 0.85, 0.30, 1.0);
// yellow ish color
pub const LIGHTNING_PROJECTILE_COLOR: Color = Color::new(1.0, 0.9, 0.2, 1.0);
// blue ish color
pub const LIGHTNING_BLOOM_COLOR: Color = Color::new(0.4, 0.7, 1.0, 1.0);
// bright white blue core
pub const LIGHTNING_CORE_COLOR: Color = Color::new(0.9, 0.95, 1.0, 1.0);
// blue-ish
pub const DNA_PROJECTILE_COLOR_1: Color = Color::new(0.18, 0.80, 1.0, 1.0);
// pink ish
pub const DNA_PROJECTILE_COLOR_2: Color = Color::new(1.0, 0.18, 0.70, 1.0);

// when triple shot it should be \ | / ish so 45 degrees on left and right
pub const TRIPLE_SHOT_SPREAD_ANGLE: f32 = std::f32::consts::FRAC_PI_4;

// proto (subordinate) enemy constants
pub const PROTO_HEALTH: i32 = 100;
pub const PROTO_RADIUS: f32 = 24.0;
pub const PROTO_COLOR: Color = Color::new(0.12, 0.53, 0.53, 1.0); // dark green-blue
pub const PROTO_BEAM_COLOR: Color = Color::new(1.0, 0.4, 0.4, 1.0); // Warm red color to fit the beam theme
pub const PROTO_PROJECTILE_COLOR: Color = Color::new(0.30, 0.85, 0.70, 1.0); // bright green-blue
pub const PROTO_PROJECTILE_SPEED: f32 = 140.0;
pub const PROTO_SPAWN_OFFSET_X: f32 = 140.0;
pub const PROTO_CLUSTER_SPREAD: f32 = 25.0 * std::f32::consts::PI / 180.0;
pub const PROTO_BULLET_SPREAD: f32 = 6.0 * std::f32::consts::PI / 180.0;
pub const PROTO_MAX_SLOTS: usize = 5;
pub const PROTO_SPAWN_MIN_INTERVAL: f32 = 3.0;
pub const PROTO_SPAWN_MAX_INTERVAL: f32 = 5.0;
pub const PROTO_IDLE_ROTATION_SPEED: f32 = 0.4;
pub const PROTO_SPINUP_PEAK_SPEED: f32 = 10.0;
pub const PROTO_SPINUP_RAMP_UP: f32 = 0.6;
pub const PROTO_SPINUP_HOLD: f32 = 0.8;
pub const PROTO_SPINUP_RAMP_DOWN: f32 = 1.0;
pub const PROTO_SPINUP_DURATION: f32 =
    PROTO_SPINUP_RAMP_UP + PROTO_SPINUP_HOLD + PROTO_SPINUP_RAMP_DOWN;
pub const PROTO_DEATH_INITIAL_DROP_SPEED: f32 = 30.0;
pub const PROTO_DEATH_GRAVITY: f32 = 1200.0;
pub const PROTO_DEATH_SPIN_HOLD: f32 = 5.0;
pub const PROTO_BEAM_SPEED: f32 = 175.0;
