mod treasure;
mod roster;
mod unit;
mod equipment;


fn main() {

    startup();
}

fn startup(){
    // initialize Roster
    let roster = roster::Roster::new();
    roster.print_units();
    // initialize Treasure
    let treasure = treasure::Treasure::new();
    treasure.print_items();
}