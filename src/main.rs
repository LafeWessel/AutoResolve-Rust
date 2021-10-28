use crate::equipment::equipment_type;

mod treasure;
mod roster;
mod unit;
mod equipment;

fn main() {
    setup();
}

fn setup(){
    // initialize Roster
    let roster = roster::Roster::new();

    // initialize Treasure
    let treasure = treasure::Treasure::new();

}