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

impl Clone for Equipment{
    fn clone(&self) -> Self {
        Equipment{
            equipment_type: self.equipment_type.clone(),
            name: self.name.clone(),
            effect: self.effect.clone(),
            coin_value: self.coin_value,
            index: self.index,
            autoresolve_bonus: self.autoresolve_bonus,
            range: self.range,
            dragon_equipment: self.dragon_equipment,
        }
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

    /// Get dragon_equipment
    pub fn get_is_dragon(&self) -> bool{
        self.dragon_equipment
    }

    /// Get equipment name
    pub fn get_name(&self) -> &str{
        self.name.as_str()
    }

}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum equipment_type{
    Armor = 1,
    Weapon,
    Trinket,
    Banner,
    Follower
}