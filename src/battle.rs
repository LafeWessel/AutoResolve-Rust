use crate::treasure::Treasure;
use crate::player::Player;
use crate::monster::monster_type;
use std::borrow::Borrow;
use crate::equipment::Equipment;

struct Battle<'a>{
    battle_type : battle_type,
    attacker : Player,
    defender : Player,
    treasure : &'a Treasure,
}

impl Battle<'_>{
    // TODO implement autoresolve()
    fn autoresolve(&self){
        let outcome = self.calculate_outcome();
        let casualties = self.calculate_casualties(&outcome);
        self.assign_casualties(&casualties);
        let treasure_results = self.treasure_results(&outcome);

    }


    //TODO implement calculate_outcome()
    fn calculate_outcome(&self) -> battle_outcome{
        let mut total : f32 = 0.0;

        // get player autoresolve bonuses
        total += self.attacker.get_autoresolve_bonus() as f32;
        total -= self.defender.get_autoresolve_bonus() as f32;
        // add random bonuses
        total += self.battle_randoms() as f32;
        total -= self.battle_randoms() as f32;

        // calculate RPS bonuses
        total += 1.5 * (self.attacker.get_cavalry_bonus() - self.defender.get_ranged_bonus()) as f32;
        total += 1.5 * (self.attacker.get_melee_bonus() - self.defender.get_cavalry_bonus()) as f32;
        total += 1.5 * (self.attacker.get_ranged_bonus() - self.defender.get_melee_bonus()) as f32;

        // add battle_type bonuses
        total += self.battle_type.get_calculation() as f32;

        // determine outcome
        battle_outcome::determine_outcome(total)

    }

    // TODO implement calculate_casualties()
    fn calculate_casualties(&self, outcome : &battle_outcome) -> battle_casualties{
        battle_casualties{}
    }
    // TODO implement assign_casualties()
    fn assign_casualties(&self, casualties : &battle_casualties){

    }
    // TODO implement battle_output()
    fn battle_output(&self){

    }
    // TODO implement treasure_results()
    fn treasure_results(&self, outcome: &battle_outcome) -> treasure_results{
        treasure_results::None
    }

    // TODO implement battle_randoms()
    fn battle_randoms(&self) -> i32
    {
        0
    }
}

struct battle_casualties{

}

enum treasure_results<'a>{
    None,
    Attacker{reward: &'a Equipment},
    Defender{reward: &'a Equipment},
    Both{attacker: &'a Equipment, defender: &'a Equipment}
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

impl battle_outcome{
    // TODO refactor to match statment
    fn determine_outcome(result : f32) -> battle_outcome{
        //All results are in relation to the attacker.
        //Victory
        if result > 2.0 {
            if result >= 20.0 {
                return battle_outcome::DecisiveVictory;
            }
            if result >= 10.0 {
                return battle_outcome::HeroicVictory;
            }
            return battle_outcome::CloseVictory;
        }
        //Defeat
        if result < -2.0 {
            if result <= -20.0 {
                return battle_outcome::CrushingDefeat;
            }
            if result <= -10.0 {
                return battle_outcome::ValiantDefeat;
            }
            return battle_outcome::CloseDefeat;
        }
        //Draw
        battle_outcome::Draw

    }
}

// TODO write unit tests