use crate::equipment::{Equipment, EquipmentType};
use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::treasure::Treasure;

#[derive(Debug)]
pub struct General{
    state : GeneralState,
    armor : Option<Equipment>,
    weapon : Option<Equipment>,
    banner : Option<Equipment>,
    follower : Option<Equipment>,
    trinket : Option<Equipment>,
    rank : i32,
    bonus : i32,
}

impl Default for General{
    fn default() -> Self {
        General::new(None, None, None, None, None, 0)
    }
}

impl Clone for General{
    fn clone(&self) -> Self {
        General{
            state: GeneralState::Unharmed,
            armor: self.armor.clone(),
            weapon: self.weapon.clone(),
            banner: self.banner.clone(),
            follower: self.follower.clone(),
            trinket: self.trinket.clone(),
            rank: self.rank,
            bonus: self.bonus
        }
    }
}

impl General{
    pub fn new(armor :  Option<Equipment>, weapon : Option<Equipment>, banner: Option<Equipment>,
               follower : Option<Equipment>, trinket : Option<Equipment>, rank : i32) -> Self{
        let mut g = General{
            state: GeneralState::Unharmed,
            armor: armor,
            weapon: weapon,
            banner: banner,
            follower: follower,
            trinket: trinket,
            rank: rank,
            bonus: 0,
        };
        g.update_bonus();
        g
    }

    /// Set piece of equipment based on equipment type
    pub fn set_equipment(&mut self, item : Equipment){
        match item.equip_type(){
            EquipmentType::Armor => self.armor = Some(item),
            EquipmentType::Weapon => self.weapon = Some(item),
            EquipmentType::Banner => self.banner = Some(item),
            EquipmentType::Trinket => self.trinket = Some(item),
            EquipmentType::Follower => self.follower = Some(item),
        };
        self.update_bonus();
    }

    /// Get current piece of equipment based on type
    pub fn get_equipment(&self, equip_type : EquipmentType) -> Option<&Equipment>{
        match equip_type{
            EquipmentType::Armor => self.armor.as_ref().map(|e| e).or_else(|| None),
            EquipmentType::Weapon => self.weapon.as_ref().map(|e| e).or_else(|| None),
            EquipmentType::Banner => self.banner.as_ref().map(|e| e).or_else(|| None),
            EquipmentType::Trinket => self.trinket.as_ref().map(|e| e).or_else(|| None),
            EquipmentType::Follower => self.follower.as_ref().map(|e| e).or_else(|| None),
        }
    }

    /// Change general state
    pub fn change_state(&mut self, new_state: GeneralState){
        self.state = new_state;
    }

    /// Get general state
    pub fn get_state(&self) -> &GeneralState {
        &self.state
    }

    /// Get rank
    pub fn get_rank(&self) -> i32{
        self.rank
    }

    /// Update bonus based on equipment and rank
    fn update_bonus(&mut self){

        self.bonus = self.armor.as_ref().map(|a| a.get_bonus()).unwrap_or_else(|| 0) +
            self.weapon.as_ref().map(|a| a.get_bonus()).unwrap_or_else(|| 0) +
            self.banner.as_ref().map(|a| a.get_bonus()).unwrap_or_else(|| 0) +
            self.trinket.as_ref().map(|a| a.get_bonus()).unwrap_or_else(|| 0) +
            self.rank;
    }

    /// Get autoresolve bonus
    pub fn get_bonus(&self) -> i32{
        self.bonus
    }

    /// Generate a General with random equipment and rank
    pub fn generate_random_general(equipment_ratio : u32, rank_cap : u32, treasure: &Treasure) -> Self{
        let mut rng = rand::thread_rng();

        General::new(
            match rng.gen_range(1..equipment_ratio+1) {
            1 => Some(treasure.get_item_by_type(EquipmentType::Armor).clone()),
            _ => None
        }, match rng.gen_range(1..equipment_ratio+1) {
            1 => Some(treasure.get_item_by_type(EquipmentType::Weapon).clone()),
            _ => None
        }, match rng.gen_range(1..equipment_ratio+1) {
            1 => Some(treasure.get_item_by_type(EquipmentType::Banner).clone()),
            _ => None
        }, match rng.gen_range(1..equipment_ratio+1) {
            1 => Some(treasure.get_item_by_type(EquipmentType::Follower).clone()),
            _ => None
        }, match rng.gen_range(1..equipment_ratio+1) {
            1 => Some(treasure.get_item_by_type(EquipmentType::Trinket).clone()),
            _ => None
        }, rng.gen_range(1..rank_cap+1) as i32
        )
    }

}

/// Holds General struct in a format for serializing/deserializing
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralJSONObject{
    armor : i32,
    weapon : i32,
    banner : i32,
    follower : i32,
    trinket : i32,
    rank : i32,
}

impl GeneralJSONObject{
    /// Produce General object from self
    pub fn produce_general(self, treasure: &Treasure) -> General{
        General::new(
            Self::get_equipment(self.armor, treasure),
            Self::get_equipment(self.weapon, treasure),
            Self::get_equipment(self.banner, treasure),
            Self::get_equipment(self.follower, treasure),
            Self::get_equipment(self.trinket, treasure),
            self.rank,
        )
    }
    fn get_equipment( id : i32, treasure : &Treasure) -> Option<Equipment>{
        if id <= 0 {
            None
        }else{
            treasure.get_item_by_id(id).map(|e| e.clone())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GeneralState {
    Unharmed,
    Wounded,
    Slain,
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::treasure::Treasure;

    #[test]
    fn test_set_equipment(){
        let r = Treasure::new(Option::None);
        let mut g = General::default();

        g.set_equipment(r.get_item_by_type(EquipmentType::Armor).clone());
        assert_eq!(EquipmentType::Armor, *g.get_equipment(EquipmentType::Armor).unwrap().equip_type());
        g.set_equipment(r.get_item_by_type(EquipmentType::Weapon).clone());
        assert_eq!(EquipmentType::Weapon, *g.get_equipment(EquipmentType::Weapon).unwrap().equip_type());
        g.set_equipment(r.get_item_by_type(EquipmentType::Banner).clone());
        assert_eq!(EquipmentType::Banner, *g.get_equipment(EquipmentType::Banner).unwrap().equip_type());
        g.set_equipment(r.get_item_by_type(EquipmentType::Trinket).clone());
        assert_eq!(EquipmentType::Trinket, *g.get_equipment(EquipmentType::Trinket).unwrap().equip_type());
        g.set_equipment(r.get_item_by_type(EquipmentType::Follower).clone());
        assert_eq!(EquipmentType::Follower, *g.get_equipment(EquipmentType::Follower).unwrap().equip_type());

    }

    #[test]
    fn test_update_bonus(){
        let mut g = General::default();
        assert_eq!(0,g.get_bonus());

        g.rank = 1;
        g.update_bonus();
        assert_eq!(g.get_bonus(), 1);
    }
}