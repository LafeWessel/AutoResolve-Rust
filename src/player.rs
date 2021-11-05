use crate::unit::{Unit, unit_type};
use crate::general::General;
use crate::faction::faction;

pub struct Player{
    units: Vec<Unit>,
    gen: General,
    faction : faction,
    melee_bonus : i32,
    cavalry_bonus : i32,
    ranged_bonus : i32,
    leader_bonus : i32,
    reinforcements  : i32,
    adv_combat : bool,
}

impl Default for Player{
    fn default() -> Self {
        Player::new(vec![],General::default())
    }
}


impl Player{
    pub fn new(units : Vec<Unit>, general : General) -> Self{
        let mut p = Player{
            units: units,
            gen : general,
            faction: faction::Rebel,
            melee_bonus: 0,
            cavalry_bonus: 0,
            ranged_bonus: 0,
            leader_bonus: 0,
            reinforcements: 0,
            adv_combat: false
        };
        p.calculate_bonuses();
        p
    }

    /// Calculate autoresolve bonuses for each type of unit
    fn calculate_bonuses(&mut self){
        self.melee_bonus = self.units.iter()
            .filter(|u| *u.get_type() == unit_type::Melee)
            .map(|u| u.get_bonus())
            .sum::<i32>();
        self.cavalry_bonus = self.units.iter()
            .filter(|u| *u.get_type() == unit_type::Cavalry)
            .map(|u| u.get_bonus())
            .sum::<i32>();
        self.ranged_bonus = self.units.iter()
            .filter(|u| *u.get_type() == unit_type::Ranged)
            .map(|u| u.get_bonus())
            .sum::<i32>();
        self.leader_bonus = self.gen.get_bonus() + if self.adv_combat {5} else {0};
    }

    /// Get number of soldiers Player has
    pub fn get_soldier_count(&self) -> i32{
        self.units.iter().map(|u| u.get_size()).sum::<i32>()
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_soldier_count(){
        let g = General::default();
        let u = Unit::new(1,String::new(),1,0,10);
        let p = Player::new(vec![u.clone()],g);

        assert_eq!(u.get_size(),p.get_soldier_count());
    }

    #[test]
    fn test_calculate_bonuses(){
        // Melee
        let g = General::default();
        let u = Unit::new(1,String::new(),1,10,0);
        let p = Player::new(vec![u.clone()], g);
        assert_eq!(p.melee_bonus, u.get_bonus());
        assert_eq!(p.cavalry_bonus, 0);
        assert_eq!(p.ranged_bonus, 0);

        // Cavalry
        let g = General::default();
        let u = Unit::new(1,String::new(),2,10,0);
        let p = Player::new(vec![u.clone()], g);
        assert_eq!(p.cavalry_bonus, u.get_bonus());
        assert_eq!(p.melee_bonus, 0);
        assert_eq!(p.ranged_bonus, 0);

        // Ranged
        let g = General::default();
        let u = Unit::new(1,String::new(),3,10,0);
        let p = Player::new(vec![u.clone()], g);
        assert_eq!(p.ranged_bonus, u.get_bonus());
        assert_eq!(p.cavalry_bonus, 0);
        assert_eq!(p.melee_bonus, 0);

    }
}