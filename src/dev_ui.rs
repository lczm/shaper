use egui_macroquad::egui;
use macroquad::prelude::{get_fps, get_frame_time};

use crate::arena::Arena;
use crate::projectile::{Projectile, ProjectileKind};
use crate::state::GameState;
use crate::world::GameEvent;

// scale to make it bigger
const DEV_UI_SCALE: f32 = 2.0;

pub fn update(state: &GameState, arena: &Arena, events: &mut Vec<GameEvent>) -> bool {
    let mut wants_pointer = false;
    egui_macroquad::ui(|ctx| {
        ctx.set_pixels_per_point(DEV_UI_SCALE);

        egui::Window::new("Shaper Dev")
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
            .default_width(240.0)
            .show(ctx, |ui| {
                ui.label(format!(
                    "fps: {}  ({:.1} ms)",
                    get_fps(),
                    get_frame_time() * 1000.0
                ));
                ui.separator();

                let (boss_current, boss_total) = arena.boss_health();
                ui.label(format!("boss hp: {boss_current} / {boss_total}"));
                ui.separator();

                let player_damage = arena.player_damage();
                ui.label(format!("lives: {}", state.lives));
                ui.label(format!("damage: {player_damage}"));
                ui.label(format!("bombs: {}", state.bombs));
                ui.separator();

                if ui.button("Trigger Level Up").clicked() {
                    events.push(GameEvent::LevelUp {
                        options: crate::level_window::generate_placeholder_options(),
                    });
                }
                ui.separator();

                let total = state.projectiles.len();
                let boss = state
                    .projectiles
                    .iter()
                    .filter(
                        |p| matches!(p, Projectile::Bullet(b) if b.kind == ProjectileKind::Boss),
                    )
                    .count();
                let beams = state
                    .projectiles
                    .iter()
                    .filter(|p| matches!(p, Projectile::Beam(_)))
                    .count();
                let player = total - boss - beams;
                ui.label(format!(
                    "projectiles: {total}  (boss {boss}, player {player}, beam {beams})"
                ));

                // draw_projectile_list(ui, state);
            });

        wants_pointer = ctx.wants_pointer_input();
    });
    wants_pointer
}

pub fn draw() {
    egui_macroquad::draw();
}

#[allow(dead_code)]
fn draw_projectile_list(ui: &mut egui::Ui, state: &GameState) {
    egui::ScrollArea::vertical()
        .max_height(280.0)
        .show(ui, |ui| {
            for (i, p) in state.projectiles.iter().enumerate() {
                let (kind, pos) = match p {
                    Projectile::Bullet(b) => {
                        let kind = match b.kind {
                            ProjectileKind::Boss => "boss",
                            ProjectileKind::Player { .. } => "player",
                        };
                        (kind, b.position)
                    }
                    Projectile::Beam(beam) => ("beam", beam.start),
                };
                ui.monospace(format!(
                    "#{i:<3} {kind:<6} ({:>5.0}, {:>5.0})",
                    pos.x, pos.y
                ));
            }
        });
}
