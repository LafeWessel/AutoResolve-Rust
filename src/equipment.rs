
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Equipment{
    equipment_type : EquipmentType,
    name : String,
    effect : String,
    coin_value : i32,
    id: i32,
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
    pub fn new(equipment_type_str: &str, name : String, effect : String, coin_value : i32, id: i32, bonus : i32, range: i32, dragon : bool) -> Self{
        Equipment{
            equipment_type: match equipment_type_str{
                "Armor" => EquipmentType::Armor,
                "Weapon" => EquipmentType::Weapon,
                "Trinket" => EquipmentType::Trinket,
                "Banner" => EquipmentType::Banner,
                "Follower" => EquipmentType::Follower,
                _ => panic!("Unable to convert {} to EquipmentType", equipment_type_str)
            },
            name,
            effect,
            coin_value,
            id,
            autoresolve_bonus: bonus,
            range,
            dragon_equipment : dragon,
        }
    }

    /// Get equipment type
    pub fn equip_type(&self) -> &EquipmentType {
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

    /// Get equipment id
    pub fn get_id(&self) -> i32{
        self.id
    }

}

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
pub enum EquipmentType {
    Armor = 1,
    Weapon,
    Trinket,
    Banner,
    Follower
}