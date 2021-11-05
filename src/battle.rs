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
    fn calculate(&self) -> battle_outcome{
        battle_outcome::Draw
    }



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
    /// Get supplies
    pub fn get_supplies(&self) -> i32{
        self.supplies
    }

    /// Get town defenses
    pub fn get_defenses(&self) -> &town_defenses{
        &self.defenses
    }
}

#[derive(Debug,Eq, PartialEq)]
enum town_defenses{
    None = 1,
    WoodenWall,
    WoodenWallAndMoat,
    StoneWall,
    StoneWallAndMoat,
}

#[derive(Debug,Eq, PartialEq)]
enum battle_outcome{
    DecisiveVictory = 1,
    HeroicVictory,
    CloseVictory,
    Draw,
    CloseDefeat,
    ValiantDefeat,
    CrushingDefeat,
}