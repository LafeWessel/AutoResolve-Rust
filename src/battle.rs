use crate::treasure::Treasure;
use crate::player::Player;
use crate::monster::monster_type;
use std::borrow::Borrow;

struct Battle<'a>{
    battle_type : battle_type,
    attacker : Player,
    defender : Player,
    treasure : &'a Treasure,
}

impl Battle<'_>{
    //TODO implement calculate_outcome()
    fn calculate_outcome(&self) -> battle_outcome{
        battle_outcome::Draw
    }

    // TODO implement calculate_casualties()

    // TODO implement assign_casualties()

    // TODO implement battle_output()

    // TODO implement treasure_results()

}


#[derive(Debug)]
enum battle_type{
    Normal{},
    Siege{rams: i32, catapults: i32, siege_towers: i32, defenses: town_stats},
    Raid{defenses: town_stats},
    Naval{attacker_ships: i32, defender_ships: i32},
    Monster{monster : monster_type}
}

impl battle_type{
    /// Calculate the autoresolve modifier for the type of battle
    fn get_calculation(&self) -> i32{
        match &self{
            battle_type::Normal { .. } => 0,
            battle_type::Siege { rams,catapults,siege_towers,defenses } => (rams * 2) + (catapults * 3) + (siege_towers * 4) - defenses.get_autoresolve_bonus(),
            battle_type::Raid { defenses } => -1 * defenses.get_autoresolve_bonus(),
            battle_type::Naval { attacker_ships,defender_ships} => 3*(attacker_ships - defender_ships),
            battle_type::Monster { monster} => monster.autoresolve_value(),
        }
    }
}

#[derive(Debug)]
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

    /// Get town defense autoresolve bonus
    pub fn get_autoresolve_bonus(&self) -> i32{
        (self.defenses as i32 * 10) - 10
    }
}

#[derive(Debug,Eq, PartialEq, Copy, Clone)]
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

// TODO write unit tests