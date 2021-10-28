#[derive(Debug)]
pub struct Equipment{
    equipment_type : equipment_type,
    name : String,
    effect : String,
    coin_value : i32,
    index : usize,
    autoresolve_bonus: i32,
    range : i32,
}

impl Equipment{
    pub fn new(equipment_type_str: &str, name : String, effect : String, value : i32, index : usize,
    bonus : i32, range: i32) -> Self{
        Equipment{
            equipment_type: match equipment_type_str{
                "Armor" => equipment_type::Armor,
                "Weapon" => equipment_type::Weapon,
                "Trinket" => equipment_type::Trinket,
                "Banner" => equipment_type::Banner,
                "Dragon" => equipment_type::Dragon,
                "Follower" => equipment_type::Follower,
                _ => panic!(format!("Unable to convert {} to equipment_type", equipment_type_str))
            },
            name: name,
            effect: effect,
            coin_value: value,
            index: index,
            autoresolve_bonus: bonus,
            range: range,
        }
    }
}

#[derive(Debug)]
enum equipment_type{
    Armor,
    Weapon,
    Trinket,
    Banner,
    Dragon,
    Follower
}