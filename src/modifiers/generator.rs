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
            ],
        }
    }

    pub fn generate_options(&self) -> [LevelUpOption; 3] {
        let mut pool = self.available.clone();

        // fisher yates shuffle
        let n = pool.len();
        for i in 0..n {
            let j = macroquad::rand::gen_range(i, n);
            pool.swap(i, j);
        }

        // take 3
        let mut choices = Vec::with_capacity(3);
        for m in pool.into_iter().take(3) {
            choices.push(LevelUpOption::new(m));
        }

        // if not enough will up with the no op modifier
        // todo : this could be stat boosts or something
        // todo : stat boosts would also be some percentage of showing up so not all modifiers all the time
        while choices.len() < 3 {
            choices.push(LevelUpOption::new(Modifier::None));
        }

        [choices[0].clone(), choices[1].clone(), choices[2].clone()]
    }

    pub fn remove_modifier(&mut self, modifier: &Modifier) {
        if let Some(pos) = self.available.iter().position(|m| m == modifier) {
            self.available.remove(pos);
        }
    }
}
