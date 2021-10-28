use crate::equipment::equipment_type;

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
    // roster.print_units();

    // initialize Treasure
    let treasure = treasure::Treasure::new();
    // treasure.print_items();


    println!("Armor: {:?}", treasure.get_item(equipment_type::Armor));
    println!("Weapon: {:?}", treasure.get_item(equipment_type::Weapon));
    println!("Follower: {:?}", treasure.get_item(equipment_type::Follower));
    println!("Banner: {:?}", treasure.get_item(equipment_type::Banner));
    println!("Trinket: {:?}", treasure.get_item(equipment_type::Trinket));
    println!("Dragon: {:?}", treasure.get_item(equipment_type::Dragon));


}