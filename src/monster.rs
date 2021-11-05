use crate::equipment::EquipmentType;

#[derive(Debug)]
pub enum monster_type{
    Minotaur ,
    Hobgoblin,
    Troll,
    Giant,
    Demon,
    Dragon,
}

impl monster_type{
    /// Get coin reward for given monster type
    fn coin_reward(&self) -> i32{
        match *self{
            monster_type::Minotaur => 200,
            monster_type::Hobgoblin => 300,
            monster_type::Troll => 400,
            monster_type::Giant => 500,
            monster_type::Demon => 700,
            monster_type::Dragon => 1400,
        }
    }

    /// Get autoresolve value for a given monster type
    pub fn autoresolve_value(&self) -> i32{
        match *self{
            monster_type::Minotaur => 20,
            monster_type::Hobgoblin => 30,
            monster_type::Troll => 40,
            monster_type::Giant => 50,
            monster_type::Demon => 60,
            monster_type::Dragon => 70,
        }
    }

    /// Get list of equipment types as reward for given monster type, return empty for Dragon
    fn rewards(&self) -> Vec<EquipmentType>{
        match *self{
            monster_type::Minotaur => vec![EquipmentType::Weapon],
            monster_type::Hobgoblin => vec![EquipmentType::Weapon, EquipmentType::Armor],
            monster_type::Troll => vec![EquipmentType::Weapon, EquipmentType::Trinket],
            monster_type::Giant => vec![EquipmentType::Weapon, EquipmentType::Trinket, EquipmentType::Armor],
            monster_type::Demon => vec![EquipmentType::Armor, EquipmentType::Banner],
            monster_type::Dragon => vec![],
        }
    }
}
