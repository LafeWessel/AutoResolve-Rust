use crate::equipment::{Equipment, EquipmentType};

pub struct General{
    state : GeneralState,
    armor : Equipment,
    weapon : Equipment,
    banner : Equipment,
    follower : Equipment,
    trinket : Equipment,
    rank : i32,
    bonus : i32,
}

impl Default for General{
    fn default() -> Self {
        let armor = Equipment::default();
        let weapon = Equipment::default();
        let banner = Equipment::default();
        let trinket = Equipment::default();
        let follower = Equipment::default();

        General::new(armor, weapon, banner, follower,trinket, 0)
    }
}

impl General{
    pub fn new(armor :  Equipment, weapon : Equipment, banner: Equipment,
    follower : Equipment, trinket : Equipment, rank : i32) -> Self{
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
            EquipmentType::Armor => self.armor = item,
            EquipmentType::Weapon => self.weapon = item,
            EquipmentType::Banner => self.banner = item,
            EquipmentType::Trinket => self.trinket = item,
            EquipmentType::Follower => self.follower = item,
        };
        self.update_bonus();
    }

    /// Get current piece of equipment based on type
    pub fn get_equipment(&self, equip_type : EquipmentType) -> &Equipment{
        match equip_type{
            EquipmentType::Armor => &self.armor,
            EquipmentType::Weapon => &self.weapon,
            EquipmentType::Banner => &self.banner,
            EquipmentType::Trinket => &self.trinket,
            EquipmentType::Follower => &self.follower,
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
        self.bonus = self.armor.get_bonus() +
            self.weapon.get_bonus() +
            self.banner.get_bonus() +
            self.trinket.get_bonus() +
            self.rank;
    }

    /// Get autoresolve bonus
    pub fn get_bonus(&self) -> i32{
        self.bonus
    }

}

#[derive(Debug)]
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

        g.set_equipment(r.get_item(EquipmentType::Armor).clone());
        assert_eq!(EquipmentType::Armor, *g.get_equipment(EquipmentType::Armor).equip_type());
        g.set_equipment(r.get_item(EquipmentType::Weapon).clone());
        assert_eq!(EquipmentType::Weapon, *g.get_equipment(EquipmentType::Weapon).equip_type());
        g.set_equipment(r.get_item(EquipmentType::Banner).clone());
        assert_eq!(EquipmentType::Banner, *g.get_equipment(EquipmentType::Banner).equip_type());
        g.set_equipment(r.get_item(EquipmentType::Trinket).clone());
        assert_eq!(EquipmentType::Trinket, *g.get_equipment(EquipmentType::Trinket).equip_type());
        g.set_equipment(r.get_item(EquipmentType::Follower).clone());
        assert_eq!(EquipmentType::Follower, *g.get_equipment(EquipmentType::Follower).equip_type());

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