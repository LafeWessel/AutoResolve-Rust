use crate::treasure::Treasure;
use crate::player::Player;
use crate::monster::Monster;

struct Battle<'a>{
    battle_type : battle_type<'a>,
    attacker : Player,
    defender : Player,
    treasure : &'a Treasure,
}

impl Battle{

}


//#[derive(Eq, PartialEq, Debug)]
enum battle_type<'a>{
    Normal{},
    Siege{rams: i32, catapults: i32, siege_towers: i32, defenses: town_stats},
    Raid{defenses: town_stats},
    Naval{attacker_ships: i32, defender_ships: i32},
    Monster{monster : Monster<'a>}
}


struct town_stats{
    supplies : i32,
    defenses: town_defenses,
}

impl town_stats{
    pub fn get_supplies(&self) -> i32{
        self.supplies
    }
    pub fn get_defenses(&self) -> &town_defenses{
        &self.defenses
    }
}

#[derive(Debug,Eq, PartialEq)]
enum town_defenses{
    None = 1,
    Wooden_Wall,
    Wooden_Wall_W_Moat,
    Stone_Wall,
    Stone_Wall_W_Moat,
}

