use crate::equipment::equipment_type;
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

    let cfg = config::new();
}


struct config {
    roster : Roster,
    treasure : Treasure,
}

impl config{
    pub fn new() -> Self{
        config{
            roster : Roster::new(),
            treasure : Treasure::new(),
        }
    }
}