use crate::faction::Faction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Unit{
    faction: Faction,
    name: String,
    unit_type: UnitType,
    autoresolve_bonus : i32,
    unit_size: i32,
    id : u32,
}

impl Default for Unit{
    fn default() -> Self {
        Unit::new("rebel",String::new(),"melee",0,0, 0)
    }    
}


impl Unit{
    pub fn new(faction_str: &str, name: String, unit_type_str: &str, bonus : i32, size: i32, id: u32) -> Self{
        Unit{
            faction :  match faction_str.to_lowercase().as_ref(){
                "rebel" => Faction::Rebel,
                "beladimir" => Faction::Beladimir,
                "lerastir" => Faction::Lerastir,
                "menoriad" => Faction::Menoriad,
                _ => panic!("Invalid integer to Faction {}!", faction_str)
            },
            name: name,
            unit_type: match unit_type_str.to_lowercase().as_ref(){
                "melee" => UnitType::Melee,
                "cavalry" => UnitType::Cavalry,
                "ranged" => UnitType::Ranged,
                _ => panic!("Invalid string to UnitType: {}", unit_type_str)
            },
            autoresolve_bonus: bonus,
            unit_size: size,
            id,
        }
    }

    /// Get Faction
    pub fn get_faction(&self) -> &Faction {
        &self.faction
    }

    /// Get unit type
    pub fn get_type(&self) -> &UnitType {
        &self.unit_type
    }

    /// Get autoresolve bonus
    pub fn get_bonus(&self) -> i32{
        self.autoresolve_bonus
    }

    /// Get unit size
    pub fn get_size(&self) -> i32{
        self.unit_size
    }

    /// Get unit name
    pub fn get_name(&self) -> &str{
        &self.name
    }

    /// Get unit id
    pub fn get_id(&self) -> u32{
        self.id
    }

    /// Assign casualties to unit, return if operation successful
    pub fn assign_casualties(&mut self, cas : i32) -> bool{
        if cas > self.unit_size{
            return false;
        }
        self.unit_size -= cas;
        true
    }

}

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
pub enum UnitType {
    Melee,
    Cavalry,
    Ranged,
}
