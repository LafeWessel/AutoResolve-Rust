use crate::unit::{Unit, UnitType};
use crate::general::{General, GeneralJSONObject};
use crate::faction::Faction;
use serde::{Deserialize, Serialize};
use crate::roster::Roster;
use rand::Rng;
use crate::treasure::Treasure;

#[derive(Debug)]
pub struct Player{
    units: Vec<Unit>,
    gen: General,
    reinforcements  : i32,
    adv_combat : bool,
    faction : Faction,

    // Bonuses
    melee_bonus : i32,
    cavalry_bonus : i32,
    ranged_bonus : i32,
    leader_bonus : i32,
}

impl Default for Player{
    fn default() -> Self {
        Player::new(vec![],General::default())
    }
}

impl Clone for Player{
    fn clone(&self) -> Self {
        Player{
            units: self.units.clone(),
            gen: self.gen.clone(),
            reinforcements: self.reinforcements,
            adv_combat: self.adv_combat,
            faction: self.faction,
            melee_bonus: self.melee_bonus,
            cavalry_bonus: self.cavalry_bonus,
            ranged_bonus: self.ranged_bonus,
            leader_bonus: self.ranged_bonus
        }
    }
}

impl Player{
    pub fn new(units : Vec<Unit>, general : General) -> Self{
        let mut p = Player{
            units: units,
            gen : general,
            faction: Faction::Rebel,
            reinforcements: 0,
            adv_combat: false,
            melee_bonus: 0,
            cavalry_bonus: 0,
            ranged_bonus: 0,
            leader_bonus: 0,
        };
        p.calculate_bonuses();
        p
    }

    pub fn new_filled(units: Vec<Unit>, gen: General, faction : Faction,
                      reinforcements : i32, adv_combat: bool, ) -> Player{
        let mut p = Player{
            units,
            gen,
            reinforcements,
            adv_combat,
            faction,
            melee_bonus: 0,
            cavalry_bonus: 0,
            ranged_bonus: 0,
            leader_bonus: 0
        };
        p.calculate_bonuses();
        p
    }

    /// Calculate autoresolve bonuses for each type of unit
    fn calculate_bonuses(&mut self){
        self.melee_bonus = self.units.iter()
            .filter(|u| *u.get_type() == UnitType::Melee)
            .map(|u| u.get_bonus())
            .sum::<i32>() + (4 * self.reinforcements);
        self.cavalry_bonus = self.units.iter()
            .filter(|u| *u.get_type() == UnitType::Cavalry)
            .map(|u| u.get_bonus())
            .sum::<i32>() + (4 * self.reinforcements);
        self.ranged_bonus = self.units.iter()
            .filter(|u| *u.get_type() == UnitType::Ranged)
            .map(|u| u.get_bonus())
            .sum::<i32>() + (4 * self.reinforcements);
        self.leader_bonus = self.gen.get_bonus() + if self.adv_combat {5} else {0};
    }

    /// Get melee bonus
    pub fn get_melee_bonus(&self) -> i32{
        self.melee_bonus
    }

    /// Get cavalry bonus
    pub fn get_cavalry_bonus(&self) -> i32{
        self.cavalry_bonus
    }

    /// Get ranged bonus
    pub fn get_ranged_bonus(&self) -> i32{
        self.ranged_bonus
    }

    /// Get number of soldiers Player has
    pub fn get_soldier_count(&self) -> i32{
        self.units.iter().map(|u| u.get_size()).sum::<i32>()
    }

    /// Get overall autoresolve bonus
    pub fn get_autoresolve_bonus(&self) -> i32
    {
        self.leader_bonus + self.melee_bonus + self.ranged_bonus + self.cavalry_bonus
    }

    /// Get reinforcements
    pub fn get_reinforcements(&self) -> i32{
        self.reinforcements
    }

    /// Get Faction
    pub fn get_faction(&self) -> &Faction{
        &self.faction
    }

    /// Get general
    pub fn get_general(&self) -> &General{
        &self.gen
    }

    /// Get mutable vector of units
    pub fn get_units_mut(&mut self) -> &mut Vec<Unit>{
        &mut self.units
    }

    /// Get vector of units
    pub fn get_units(& self) -> &Vec<Unit>{
        &self.units
    }

    /// Get advanced combat deck
    pub fn has_advanced_combat_deck(&self) -> bool{
        self.adv_combat
    }

    /// Get unit count by name
    pub fn get_unit_count_by_name(&self, name : &str) -> i32{
        self.units.iter().filter(|u| u.get_name() == name).collect::<Vec<&Unit>>().len() as i32
    }

    /// Generate a Player with random values
    pub fn generate_random_player(equipment_ratio: u32, rank_cap:u32, roster : &Roster, reinforcement_cap : u32, treasure : &Treasure) -> Self{
        let mut rng = rand::thread_rng();

        let gen = General::generate_random_general(equipment_ratio, rank_cap, treasure);
        let fac = Faction::generate_random_faction();
        let rein = rng.gen_range(0..=reinforcement_cap);
        let adv = rng.gen::<bool>();
        let mut units: Vec<Unit> = vec![];

        let faction_roster = roster.get_faction_roster(fac);

        for i in 1..rng.gen_range(2..=20){
            units.push(faction_roster[rng.gen_range(0..faction_roster.len())].clone());
        }

        Player::new_filled(units, gen, fac, rein as i32, adv)
    }
}

/// Holds Player struct in a format for serializing/deserializing
#[derive(Debug,Deserialize,Serialize)]
pub struct PlayerJSONObject{
    general : GeneralJSONObject,
    units : Vec<u32>,
    reinforcements : i32,
    adv_combat : bool,
    faction : Faction,
}

impl PlayerJSONObject{
    /// Produce Player object from self
    pub fn produce_player(self, roster : &Roster, treasure : &Treasure) -> Player{
        Player::new_filled(
            self.units.iter().map(|i| roster.get_unit_by_id(*i).clone()).collect::<Vec<Unit>>(),
            self.general.produce_general(treasure),
            self.faction,
            self.reinforcements,
            self.adv_combat
        )
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_soldier_count(){
        let g = General::default();
        let u = Unit::new("rebel",String::new(),"melee",0,10,0);
        let p = Player::new(vec![u.clone()],g);

        assert_eq!(u.get_size(),p.get_soldier_count());
    }

    #[test]
    fn test_calculate_bonuses(){
        // Melee
        let g = General::default();
        let u = Unit::new("rebel",String::new(),"Melee",10,0,0);
        let p = Player::new(vec![u.clone()], g);
        assert_eq!(p.melee_bonus, u.get_bonus());
        assert_eq!(p.cavalry_bonus, 0);
        assert_eq!(p.ranged_bonus, 0);

        // Cavalry
        let g = General::default();
        let u = Unit::new("rebel",String::new(),"Cavalry",10,0,0);
        let p = Player::new(vec![u.clone()], g);
        assert_eq!(p.cavalry_bonus, u.get_bonus());
        assert_eq!(p.melee_bonus, 0);
        assert_eq!(p.ranged_bonus, 0);

        // Ranged
        let g = General::default();
        let u = Unit::new("rebel",String::new(),"Ranged",10,0,0);
        let p = Player::new(vec![u.clone()], g);
        assert_eq!(p.ranged_bonus, u.get_bonus());
        assert_eq!(p.cavalry_bonus, 0);
        assert_eq!(p.melee_bonus, 0);

    }

    #[test]
    fn reinforcement_test(){
        let g = General::default();
        let p = Player::new_filled(vec![],g, Faction::Rebel,1,false);

        assert_eq!(4,p.melee_bonus);
        assert_eq!(4,p.cavalry_bonus);
        assert_eq!(4,p.ranged_bonus);
    }

    #[test]
    fn advanced_combat_deck_test(){
        let g = General::default();
        let p = Player::new_filled(vec![],g, Faction::Rebel,0,true);

        assert_eq!(5, p.leader_bonus);
    }

    #[test]
    fn test_get_unit_count_by_name(){
        let g = General::default();
        let u1 = Unit::new("rebel",String::from("test1"),"Cavalry",10,0,0);
        let u2 = Unit::new("rebel",String::from("test1"),"Cavalry",10,0,0);
        let u3 = Unit::new("rebel",String::from("test2"),"Cavalry",10,0,0);
        let p = Player::new(vec![u1,u2,u3], g);

        assert_eq!(2,p.get_unit_count_by_name("test1"));
        assert_eq!(1,p.get_unit_count_by_name("test2"));
        assert_eq!(0,p.get_unit_count_by_name("test3"));

    }
}