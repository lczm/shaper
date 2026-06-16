use egui_macroquad::egui;

use crate::projectile::ProjectileKind;
use crate::state::GameState;

// scale to make it bigger
const DEV_UI_SCALE: f32 = 2.0;

pub fn draw(state: &GameState) {
    egui_macroquad::ui(|ctx| {
        ctx.set_pixels_per_point(DEV_UI_SCALE);

        egui::Window::new("dev")
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
            .default_width(240.0)
            .show(ctx, |ui| {
                ui.label(format!("lives: {}", state.lives));
                ui.separator();

                let total = state.projectiles.len();
                let boss = state
                    .projectiles
                    .iter()
                    .filter(|p| p.kind == ProjectileKind::Boss)
                    .count();
                let player = total - boss;
                ui.label(format!(
                    "projectiles: {total}  (boss {boss}, player {player})"
                ));

                egui::ScrollArea::vertical()
                    .max_height(280.0)
                    .show(ui, |ui| {
                        for (i, p) in state.projectiles.iter().enumerate() {
                            let kind = match p.kind {
                                ProjectileKind::Boss => "boss",
                                ProjectileKind::Player => "player",
                            };
                            ui.monospace(format!(
                                "#{i:<3} {kind:<6} ({:>5.0}, {:>5.0})",
                                p.position.x, p.position.y
                            ));
                        }
                    });
            });
    });
    egui_macroquad::draw();
}
