use crate::equipment::EquipmentType;
use serde::{Deserialize,Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq,Deserialize,Serialize)]
pub enum MonsterType {
    Minotaur ,
    Hobgoblin,
    Troll,
    Giant,
    Demon,
    Dragon,
}

impl MonsterType {
    /// Get coin reward for given monster type
    fn coin_reward(&self) -> i32{
        match *self{
            MonsterType::Minotaur => 200,
            MonsterType::Hobgoblin => 300,
            MonsterType::Troll => 400,
            MonsterType::Giant => 500,
            MonsterType::Demon => 700,
            MonsterType::Dragon => 1400,
        }
    }

    /// Get autoresolve value for a given monster type
    pub fn autoresolve_value(&self) -> i32{
        match *self{
            MonsterType::Minotaur => 20,
            MonsterType::Hobgoblin => 30,
            MonsterType::Troll => 40,
            MonsterType::Giant => 50,
            MonsterType::Demon => 60,
            MonsterType::Dragon => 70,
        }
    }

    /// Get list of equipment types as reward for given monster type, return empty for Dragon
    fn rewards(&self) -> Vec<EquipmentType>{
        match *self{
            MonsterType::Minotaur => vec![EquipmentType::Weapon],
            MonsterType::Hobgoblin => vec![EquipmentType::Weapon, EquipmentType::Armor],
            MonsterType::Troll => vec![EquipmentType::Weapon, EquipmentType::Trinket],
            MonsterType::Giant => vec![EquipmentType::Weapon, EquipmentType::Trinket, EquipmentType::Armor],
            MonsterType::Demon => vec![EquipmentType::Armor, EquipmentType::Banner],
            MonsterType::Dragon => vec![],
        }
    }
}
