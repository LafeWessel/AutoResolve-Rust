use crate::unit::Unit;
use crate::general::General;
use crate::faction::faction;

struct Player{
    units: Vec<Unit>,
    gen: General,
    faction : faction,
    melee_bonus : i32,
    cavalry_bonus : i32,
    ranged_bonus : i32,
    reinforcements  : i32,
    adv_combat : bool,
}

impl Player{
    pub fn new(units : Vec<Unit>, general : General) -> Self{
        Player{
            units: units,
            gen : general,
            faction: faction::Rebel,
            melee_bonus: 0,
            cavalry_bonus: 0,
            ranged_bonus: 0,
            reinforcements: 0,
            adv_combat: false
        }
    }


}