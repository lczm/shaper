use crate::level_window::LevelUpOption;
use crate::modifiers::Modifier;

pub struct ModifiersGenerator {
    pub available: Vec<Modifier>,
}

impl Default for ModifiersGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ModifiersGenerator {
    pub fn new() -> Self {
        Self {
            available: vec![
                Modifier::Homing,
                Modifier::Bouncing,
                Modifier::Lightning,
                Modifier::Dna,
                Modifier::TripleShot,
            ],
        }
    }

    pub fn generate_options(&self, player_damage: i32) -> [LevelUpOption; 3] {
        let mut pool = self.available.clone();

        // fisher yates shuffle
        let n = pool.len();
        for i in 0..n {
            let j = macroquad::rand::gen_range(i, n);
            pool.swap(i, j);
        }

        let mut choices = Vec::with_capacity(3);

        // if there are 3 or more modifiers available
        // then 2 of the options will be modifiers, the 3rd will be randomized between a modifier or a stat upgrade
        if pool.len() >= 3 {
            choices.push(LevelUpOption::new(pool[0].clone()));
            choices.push(LevelUpOption::new(pool[1].clone()));
            if macroquad::rand::gen_range(0, 2) == 0 {
                choices.push(LevelUpOption::new(pool[2].clone()));
            } else {
                choices.push(LevelUpOption::new(Self::generate_stat_upgrade(
                    player_damage,
                )));
            }
        } else if pool.len() == 2 {
            choices.push(LevelUpOption::new(pool[0].clone()));
            choices.push(LevelUpOption::new(pool[1].clone()));
            choices.push(LevelUpOption::new(Self::generate_stat_upgrade(
                player_damage,
            )));
        } else if pool.len() == 1 {
            choices.push(LevelUpOption::new(pool[0].clone()));
            choices.push(LevelUpOption::new(Self::generate_stat_upgrade(
                player_damage,
            )));
            choices.push(LevelUpOption::new(Self::generate_stat_upgrade(
                player_damage,
            )));
        } else {
            choices.push(LevelUpOption::new(Self::generate_stat_upgrade(
                player_damage,
            )));
            choices.push(LevelUpOption::new(Self::generate_stat_upgrade(
                player_damage,
            )));
            choices.push(LevelUpOption::new(Self::generate_stat_upgrade(
                player_damage,
            )));
        }

        [choices[0].clone(), choices[1].clone(), choices[2].clone()]
    }

    fn generate_stat_upgrade(player_damage: i32) -> Modifier {
        match macroquad::rand::gen_range(0, 3) {
            0 => {
                // flat damage boost: 10%-20%
                let flat_damage_boost = macroquad::rand::gen_range(10, 21);
                let boost = ((player_damage as f32) * (flat_damage_boost as f32) / 100.0)
                    .round()
                    .max(1.0) as i32;
                Modifier::DamageBoost(boost)
            }
            1 => {
                // fire rate boost: 15%-25%
                let fire_rate_boost = macroquad::rand::gen_range(15, 26);
                Modifier::FireRateBoost(fire_rate_boost)
            }
            _ => Modifier::BombBoost,
        }
    }

    pub fn remove_modifier(&mut self, modifier: &Modifier) {
        if let Some(pos) = self.available.iter().position(|m| m == modifier) {
            self.available.remove(pos);
        }
    }
}
