use crate::roster::Roster;
use crate::treasure::Treasure;

mod treasure;
mod roster;
mod unit;
mod equipment;
mod faction;
mod general;
mod player;
mod battle;
mod monster;

fn main() {

    let cfg = Config::new();
}


struct Config {
    roster : Roster,
    treasure : Treasure,
}

impl Config {
    pub fn new() -> Self{
        Config {
            roster : Roster::new(),
            treasure : Treasure::new(),
        }
    }
}

// TODO implement TestBattle: randomized testing of battles

// TODO implement BattleData: saving data about battles

// TODO implement CLI using Rust equivalent of argparse