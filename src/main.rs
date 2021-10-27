mod treasure;
mod roster;


fn main() {

    println!("Hello, world!");
}

fn startup(){
    // initialize Roster
    let roster = roster::Roster::new_and_init();
    // initialize Treasure
    let treasure = treasure::Treasure::new_and_init();
}