use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum Faction {
    Rebel,
    Beladimir,
    Lerastir,
    Menoriad,
}

impl Faction{
    /// Generate random faction
    pub fn generate_random_faction() -> Self{
        let mut rng = rand::thread_rng();

        match rng.gen_range(1..=4) {
            1 => Faction::Rebel,
            2 => Faction::Beladimir,
            3 => Faction::Lerastir,
            4 => Faction::Menoriad,
            _ => panic!("Invalid number generated")
        }
    }
}