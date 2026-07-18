use crate::modifiers::{Modifier, ModifierState};
use crate::projectile::BulletProjectile;
use macroquad::prelude::Vec2;

// stores a list of modifiers that is applied to each projectile
pub struct ProjectileRecipe {
    pub modifiers: Vec<Modifier>,
}

impl ProjectileRecipe {
    pub fn new() -> Self {
        Self { modifiers: vec![] }
    }

    pub fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.push(modifier);
    }

    // need to track total projectile spawn count
    // since it can change based on modifiers
    pub fn spawn_count(&self) -> usize {
        let mut count: usize = 1;
        for modifier in &self.modifiers {
            let (_, m) = modifier.damage_contribution(0);
            count *= m as usize;
        }
        count
    }

    // to apply a recipe to a projectile
    // we clone all the modifiers onto the projectiles
    // and run on_spawn for each of them
    pub fn apply(
        &self,
        bullet: &mut BulletProjectile,
        index: usize,
    ) -> (Vec<Modifier>, ModifierState) {
        let modifiers: Vec<Modifier> = self.modifiers.clone();
        let mut state = ModifierState::default();

        if modifiers.contains(&Modifier::Dna) {
            state.dna_phase = if index % 2 == 0 {
                0.0
            } else {
                std::f32::consts::PI
            };
        }

        // fan out if triple shot is taken
        if modifiers.contains(&Modifier::TripleShot) {
            // dna also modifies it
            let per_slot: usize = if modifiers.contains(&Modifier::Dna) {
                2
            } else {
                1
            };
            // 0, 1, or 2
            let triple_index = index / per_slot;
            let angle_offset =
                (triple_index as f32 - 1.0) * crate::constants::TRIPLE_SHOT_SPREAD_ANGLE;
            let speed = bullet.velocity.length();
            let base_angle = bullet.velocity.to_angle();
            bullet.velocity = Vec2::from_angle(base_angle + angle_offset) * speed;
        }

        for modifier in &modifiers {
            modifier.on_spawn(
                &mut state,
                &mut bullet.position,
                &mut bullet.velocity,
                &mut bullet.circle,
            );
        }

        (modifiers, state)
    }

    pub fn potential_damage(&self, base_damage: i32) -> i32 {
        let mut bonus = 0;
        let mut multiplier = 1;
        for modifier in &self.modifiers {
            let (b, m) = modifier.damage_contribution(base_damage);
            bonus += b;
            multiplier *= m;
        }
        (base_damage + bonus) * multiplier
    }
}

impl Default for ProjectileRecipe {
    fn default() -> Self {
        Self::new()
    }
}
