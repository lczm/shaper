use crate::modifiers::{Modifier, ModifierState};
use crate::projectile::BulletProjectile;

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
            state.dna_phase = if index == 0 {
                0.0
            } else {
                std::f32::consts::PI
            };
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
