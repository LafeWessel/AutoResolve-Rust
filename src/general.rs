use crate::faction::faction;
use crate::equipment::{Equipment, equipment_type};
use std::ops::Deref;
use std::borrow::Borrow;

struct General<'a>{
    state : general_state,
    armor : &'a Equipment,
    weapon : &'a Equipment,
    banner : &'a Equipment,
    follower : &'a Equipment,
    trinket : &'a Equipment,
    rank : i32,
    bonus : i32,
}

impl<'a> General<'a>{
    pub fn new(armor :  &'a Equipment, weapon : &'a Equipment, banner: &'a Equipment,
    follower : &'a Equipment, trinket : &'a Equipment, rank : i32) -> Self{
        let mut g = General{
            state: general_state::Unharmed,
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
    pub fn set_equipment(&mut self, item : &'a Equipment){
        match item.equip_type(){
            equipment_type::Armor => self.armor = item,
            equipment_type::Weapon => self.weapon = item,
            equipment_type::Banner => self.banner = item,
            equipment_type::Trinket => self.trinket = item,
            equipment_type::Follower => self.follower = item,
        };
        self.update_bonus();
    }

    /// Get current piece of equipment based on type
    pub fn get_equipment(&self, equip_type : equipment_type) -> &'a Equipment{
        match equip_type{
            equipment_type::Armor => self.armor,
            equipment_type::Weapon => self.weapon,
            equipment_type::Banner => self.banner,
            equipment_type::Trinket => self.trinket,
            equipment_type::Follower => self.follower,
        }
    }

    /// Change general state
    pub fn change_state(&mut self, new_state: general_state){
        self.state = new_state;
    }

    /// Get general state
    pub fn get_state(&self) -> &general_state{
        &self.state
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


enum general_state{
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
        let r = Treasure::new();
        let e = Equipment::default();
        let mut g = General::new(&e, &e,
                             &e, &e,
                             &e, 0);

        g.set_equipment(r.get_item(equipment_type::Armor));
        assert_eq!(equipment_type::Armor, *g.get_equipment(equipment_type::Armor).equip_type());
        g.set_equipment(r.get_item(equipment_type::Weapon));
        assert_eq!(equipment_type::Weapon, *g.get_equipment(equipment_type::Weapon).equip_type());
        g.set_equipment(r.get_item(equipment_type::Banner));
        assert_eq!(equipment_type::Banner, *g.get_equipment(equipment_type::Banner).equip_type());
        g.set_equipment(r.get_item(equipment_type::Trinket));
        assert_eq!(equipment_type::Trinket, *g.get_equipment(equipment_type::Trinket).equip_type());
        g.set_equipment(r.get_item(equipment_type::Follower));
        assert_eq!(equipment_type::Follower, *g.get_equipment(equipment_type::Follower).equip_type());

    }

    #[test]
    fn test_update_bonus(){
        let e = Equipment::default();
        let mut g = General::new(&e, &e,
                                 &e, &e,
                                 &e, 0);
        let t = Treasure::new();

        assert_eq!(0,g.get_bonus());

        g.set_equipment(t.get_item(equipment_type::Armor));
        assert!(g.get_bonus() > 0);
    }
}