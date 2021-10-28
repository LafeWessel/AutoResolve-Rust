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

}


enum general_state{
    Unharmed,
    Wounded,
    Slain,
}

