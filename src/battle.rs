use crate::treasure::Treasure;

struct Battle{

}


//#[derive(Eq, PartialEq, Debug)]
enum battle_type{
    Normal{},
    Siege{rams: i32, catapults:i32, siege_towers:i32, defenses: town_stats},
    Raid{defenses: town_stats},
    Naval{attacker_ships: i32, defender_ships: i32},
    Monster{}
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

enum town_defenses{
    None = 1,
    Wooden_Wall,
    Wooden_Wall_W_Moat,
    Stone_Wall,
    Stone_Wall_W_Moat,
}

struct Monster<'a>{
    mon_type : monster_type,
    autoresolve_bonus : i32,
    coin_reward : i32,
    treasure : &'a Treasure,
}

enum monster_type{
    Minotaur = 1,
    Hobgoblin,
    Troll,
    Giant,
    Demon,
    Dragon,
}