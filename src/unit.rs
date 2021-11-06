use crate::faction::Faction;

#[derive(Debug)]
pub struct Unit{
    faction: Faction,
    name: String,
    unit_type: UnitType,
    autoresolve_bonus : i32,
    unit_size: i32,
}

impl Default for Unit{
    fn default() -> Self {
        Unit::new(1,String::new(),1,0,0)
    }    
}

impl Clone for Unit{
    fn clone(&self) -> Self {
        Unit{
            faction: self.faction.clone(),
            name: self.name.clone(),
            unit_type: self.unit_type.clone(),
            autoresolve_bonus: self.autoresolve_bonus,
            unit_size: self.unit_size,
        }
    }
}


impl Unit{
    pub fn new(faction_int : u32, name: String, unit_type_int:u32, bonus : i32, size: i32) -> Self{
        Unit{
            faction :  match faction_int{
                1 => Faction::Rebel,
                2 => Faction::Beladimir,
                3 => Faction::Lerastir,
                4 => Faction::Menoriad,
                _ => panic!(format!{"Invalid integer to Faction {}!",faction_int})
            },
            name: name,
            unit_type: match unit_type_int{
                1 => UnitType::Melee,
                2 => UnitType::Cavalry,
                3 => UnitType::Ranged,
                _ => panic!(format!{"Invalid integer to UnitType: {}", unit_type_int})
            },
            autoresolve_bonus: bonus,
            unit_size: size,
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

    /// Assign casualties to unit, return if operation successful
    pub fn assign_casualties(&mut self, cas : i32) -> bool{
        if cas > self.unit_size{
            return false;
        }
        self.unit_size -= cas;
        true
    }

}

#[derive(Debug,Eq,PartialEq,Copy, Clone)]
pub enum UnitType {
    Melee,
    Cavalry,
    Ranged,
}
