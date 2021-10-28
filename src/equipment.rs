#[derive(Debug)]
pub struct Equipment{
    equipment_type : equipment_type,
    name : String,
    effect : String,
    coin_value : i32,
    index : usize,
    autoresolve_bonus: i32,
    range : i32,
    dragon_equipment : bool,
}

impl Default for Equipment{
    fn default() -> Self {
        Equipment::new("Armor",String::from(""),String::from(""),0,0,0,0, false)
    }
}

impl Equipment{
    pub fn new(equipment_type_str: &str, name : String, effect : String, value : i32, index : usize,
    bonus : i32, range: i32, dragon : bool) -> Self{
        Equipment{
            equipment_type: match equipment_type_str{
                "Armor" => equipment_type::Armor,
                "Weapon" => equipment_type::Weapon,
                "Trinket" => equipment_type::Trinket,
                "Banner" => equipment_type::Banner,
                "Follower" => equipment_type::Follower,
                _ => panic!(format!("Unable to convert {} to equipment_type", equipment_type_str))
            },
            name: name,
            effect: effect,
            coin_value: value,
            index: index,
            autoresolve_bonus: bonus,
            range: range,
            dragon_equipment : dragon,
        }
    }

    /// Get equipment type
    pub fn equip_type(&self) -> &equipment_type{
        &self.equipment_type
    }

    /// Get autoresolve bonus
    pub fn get_bonus(&self) -> i32{
        self.autoresolve_bonus
    }

}

#[derive(Debug, PartialEq, Eq)]
pub enum equipment_type{
    Armor = 1,
    Weapon,
    Trinket,
    Banner,
    Follower
}