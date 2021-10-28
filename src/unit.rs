use crate::faction::faction;

#[derive(Debug)]
pub struct Unit{
    faction: faction,
    name: String,
    unit_type: unit_type,
    autoresolve_bonus : i32,
    unit_size: i32,
}

impl Unit{
    pub fn new(faction_int : u32, name: String, unit_type_int:u32, bonus : i32, size: i32) -> Self{
        Unit{
            faction :  match faction_int{
                1 => faction::Rebel,
                2 => faction::Beladimir,
                3 => faction::Lerastir,
                4 => faction::Menoriad,
                _ => panic!(format!{"Invalid integer to faction {}!",faction_int})
            },
            name: name,
            unit_type: match unit_type_int{
                1 => unit_type::Melee,
                2 => unit_type::Cavalry,
                3 => unit_type::Ranged,
                _ => panic!(format!{"Invalid integer to unit_type: {}", unit_type_int})
            },
            autoresolve_bonus: bonus,
            unit_size: size,
        }
    }

    /// Get Faction
    pub fn get_faction(&self) -> &faction{
        &self.faction
    }

}

#[derive(Debug)]
enum unit_type{
    Melee,
    Cavalry,
    Ranged,
}
