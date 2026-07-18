use macroquad::prelude::*;

use crate::constants::{
    ARENA_BORDER_THICKNESS, BEAM_ACTIVE_DURATION, BEAM_BORDER_THICKNESS, BEAM_COLOR,
    BEAM_STARTUP_DURATION, BEAM_WIDTH, PROJECTILE_RADIUS,
};
use crate::gfx::Shaders;
use crate::modifiers::{Modifier, ModifierContext, ModifierState};
use crate::shape::Circle;

#[derive(Clone, Copy, PartialEq)]
pub enum ProjectileKind {
    Boss,
    Player { damage: i32 },
}

// a moving circular bullet (the original projectile)
pub struct BulletProjectile {
    pub position: Vec2,
    pub velocity: Vec2,
    pub circle: Circle,
    pub kind: ProjectileKind,

    // projectile has a list of modifiers that are applied to it
    // and each of the modifiers are composable
    pub modifiers: Vec<Modifier>,
    pub modifier_state: ModifierState,
}

impl BulletProjectile {
    pub fn new(position: Vec2, velocity: Vec2, kind: ProjectileKind, color: Color) -> Self {
        let mut circle = Circle::new(PROJECTILE_RADIUS, color);
        circle.filled = true;
        BulletProjectile {
            position,
            velocity,
            circle,
            kind,
            modifiers: vec![],
            modifier_state: ModifierState::default(),
        }
    }

    pub fn update(&mut self, dt: f32, ctx: Option<&ModifierContext>) {
        self.position += self.velocity * dt;

        // run modifier on_update hooks (player bullets only)
        if let Some(ctx) = ctx {
            for modifier in &self.modifiers {
                modifier.on_update(
                    &mut self.modifier_state,
                    &mut self.position,
                    &mut self.velocity,
                    &mut self.circle,
                    dt,
                    ctx,
                );
            }
        }
    }

    // true once the bullet's edge reaches the inner edge of the border, so it's
    // culled right at the border instead of visibly crossing it
    pub fn is_off_screen(&self, bounds: Rect) -> bool {
        let r = self.circle.radius;
        let inset = ARENA_BORDER_THICKNESS / 2.0;
        self.position.x - r < bounds.x + inset
            || self.position.x + r > bounds.x + bounds.w - inset
            || self.position.y - r < bounds.y + inset
            || self.position.y + r > bounds.y + bounds.h - inset
    }

    pub fn draw(&self) {
        self.circle.draw(self.position, 1.0);
    }
}

// a beam, starts off as an indicator and becomes active
pub struct BeamProjectile {
    pub start: Vec2,
    pub end: Vec2,
    // needed to track the -> spawn -> inactive indicator -> active -> despawn
    elapsed: f32,
}

impl BeamProjectile {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        BeamProjectile {
            start,
            end,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    // it is active only after the startup duration
    pub fn is_active(&self) -> bool {
        self.elapsed >= BEAM_STARTUP_DURATION
            && self.elapsed < BEAM_STARTUP_DURATION + BEAM_ACTIVE_DURATION
    }

    // anytime after the active window
    pub fn is_expired(&self) -> bool {
        self.elapsed >= BEAM_STARTUP_DURATION + BEAM_ACTIVE_DURATION
    }

    pub fn draw(&self, shaders: &Shaders) {
        if self.is_active() {
            // active shadow draw beam
            shaders.draw_beam(self.start, self.end, BEAM_WIDTH, BEAM_COLOR);
        } else {
            // todo : this draws a white border but make it neater
            let dir = (self.end - self.start).normalize_or_zero();
            let perp = vec2(-dir.y, dir.x);
            let inset = (BEAM_WIDTH - BEAM_BORDER_THICKNESS) / 2.0;
            for side in [-1.0, 1.0] {
                let off = perp * (inset * side);
                draw_line(
                    self.start.x + off.x,
                    self.start.y + off.y,
                    self.end.x + off.x,
                    self.end.y + off.y,
                    BEAM_BORDER_THICKNESS,
                    WHITE,
                );
            }
        }
    }
}

// either kind of projectile, stored together in the game state
pub enum Projectile {
    Bullet(BulletProjectile),
    Beam(BeamProjectile),
}

impl Projectile {
    pub fn update(&mut self, dt: f32, ctx: Option<&ModifierContext>) {
        match self {
            Projectile::Bullet(b) => b.update(dt, ctx),
            Projectile::Beam(beam) => beam.update(dt),
        }
    }

    // true when the projectile should be removed
    // the arena uses this to remove it
    pub fn is_dead(&self, bounds: Rect) -> bool {
        match self {
            Projectile::Bullet(b) => b.is_off_screen(bounds),
            Projectile::Beam(beam) => beam.is_expired(),
        }
    }

    pub fn draw(&self, shaders: &Shaders) {
        match self {
            Projectile::Bullet(b) => b.draw(),
            Projectile::Beam(beam) => beam.draw(shaders),
        }
    }
}
