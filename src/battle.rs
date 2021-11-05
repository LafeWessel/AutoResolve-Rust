use crate::treasure::Treasure;
use crate::player::Player;
use crate::monster::monster_type;
use std::borrow::Borrow;
use crate::equipment::{Equipment, equipment_type};
use crate::general::general_state;
use rand::Rng;

struct Battle<'a>{
    battle_type : BattleType,
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
        let treasure_results = self.treasure_results();

    }

    /// Calculate the outcome of the battle based on each Player's statistics
    fn calculate_outcome(&self) -> BattleOutcome {
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

        // add BattleType bonuses
        total += self.battle_type.get_calculation() as f32;

        // determine outcome
        BattleOutcome::determine_outcome(total)

    }

    /// Calculate casualties for attacker and defender based on battle outcome
    fn calculate_casualties(&self, outcome : &BattleOutcome) -> BattleCasualties {
        let mut rng = rand::thread_rng();

        // Attacker Casualties
        let att_tot = self.attacker.get_soldier_count();
        let mut att_cas = 0;
        for i in 0..att_tot/10{
            att_cas += rng.gen_range(0..((*outcome as i32) + 1));
        }
        let att_unit_cas = if (att_cas/7)-1 < 0 {0} else {(att_cas/7)-1};

        // Defender Casualties
        let def_tot = self.defender.get_soldier_count();
        let mut def_cas = 0;
        for i in 0..att_tot/10{
            def_cas += rng.gen_range(0..((*outcome as i32) + 1));
        }
        let def_unit_cas = if (def_cas/7)-1 < 0 {0} else {(def_cas/7)-1};

        // Upgrades
        let att_up = def_cas / 6;
        let def_up = att_cas / 6;

        // Attacker General state
        let mut att_gen = general_state::Unharmed;
        if rng.gen_range(1..9) <= 2{
            att_gen = general_state::Wounded;
            if rng.gen_range(1..9) <= 2{
                att_gen = general_state::Slain;
            }
        }
        // Defender General state
        let mut def_gen = general_state::Unharmed;
        if rng.gen_range(1..9) <= 2{
            def_gen = general_state::Wounded;
            if rng.gen_range(1..9) <= 2{
               def_gen = general_state::Slain;
            }
        }

        BattleCasualties {
            attacker: Some(Casualties{
                state: att_gen,
                upgrades: att_up,
                casualties: att_cas,
                unit_casualties: att_unit_cas,
            }),
            defender: Some(Casualties{
                state: def_gen,
                upgrades: def_up,
                casualties: def_cas,
                unit_casualties: def_unit_cas,
            }),
        }
    }

    // TODO implement assign_casualties()
    fn assign_casualties(&self, casualties : &BattleCasualties){

    }

    // TODO implement battle_output()
    fn battle_output(&self){

    }

    /// Determine treasure results for a battle
    fn treasure_results(&self) -> TreasureResults {
        TreasureResults {
            attacker: self.find_treasure(&self.attacker),
            defender: self.find_treasure(&self.defender),
        }
    }

    /// Determine if treasure is found by a given player
    fn find_treasure(&self, player : &Player) -> Option<&Equipment>{
        let mut rng = rand::thread_rng();
        let bonus = player.get_general().get_equipment(equipment_type::Follower).get_bonus();
        if rng.gen_range(1..9) + bonus >= 5{
            return Some(self.treasure.find_equipment());
        }
        None
    }

    /// Generate random modifiers for battle autoresolving
    fn battle_randoms(&self) -> i32
    {
        let mut rng = rand::thread_rng();
        // sum of 10x random in range 1-10
        let mut sum = 0;
        for _ in 0..10 {
            sum += rng.gen_range(1..11);
        }
        sum
    }
}

struct BattleCasualties{
    attacker : Option<Casualties>,
    defender : Option<Casualties>,
}

struct Casualties {
    state : general_state,
    upgrades : i32,
    casualties : i32,
    unit_casualties : i32,
}

struct TreasureResults<'a>{
    attacker : Option<&'a Equipment>,
    defender : Option<&'a Equipment>,
}

#[derive(Debug)]
enum BattleType {
    Normal{},
    Siege{rams: i32, catapults: i32, siege_towers: i32, defenses: TownStats },
    Raid{defenses: TownStats },
    Naval{attacker_ships: i32, defender_ships: i32},
    Monster{monster : monster_type}
}

impl BattleType {
    /// Calculate the autoresolve modifier for the type of battle
    fn get_calculation(&self) -> i32{
        match &self{
            BattleType::Normal { .. } => 0,
            BattleType::Siege { rams,catapults,siege_towers,defenses } => (rams * 2) + (catapults * 3) + (siege_towers * 4) - defenses.get_autoresolve_bonus(),
            BattleType::Raid { defenses } => -1 * defenses.get_autoresolve_bonus(),
            BattleType::Naval { attacker_ships,defender_ships} => 3*(attacker_ships - defender_ships),
            BattleType::Monster { monster} => monster.autoresolve_value(),
        }
    }
}

#[derive(Debug)]
struct TownStats {
    supplies : i32,
    defenses: TownDefenses,
}

impl TownStats {
    /// Get supplies
    pub fn get_supplies(&self) -> i32{
        self.supplies
    }

    /// Get town defenses
    pub fn get_defenses(&self) -> &TownDefenses {
        &self.defenses
    }

    /// Get town defense autoresolve bonus
    pub fn get_autoresolve_bonus(&self) -> i32{
        (self.defenses as i32 * 10) - 10
    }
}

#[derive(Debug,Eq, PartialEq, Copy, Clone)]
enum TownDefenses {
    None = 1,
    WoodenWall,
    WoodenWallAndMoat,
    StoneWall,
    StoneWallAndMoat,
}

#[derive(Debug,Eq, PartialEq, Copy, Clone)]
enum BattleOutcome {
    DecisiveVictory = 1,
    HeroicVictory,
    CloseVictory,
    Draw,
    CloseDefeat,
    ValiantDefeat,
    CrushingDefeat,
}

impl BattleOutcome {
    // TODO refactor to match statement
    /// Determine which outcome based on f32 result
    fn determine_outcome(result : f32) -> BattleOutcome {
        //All results are in relation to the attacker.
        //Victory
        if result > 2.0 {
            if result >= 20.0 {
                return BattleOutcome::DecisiveVictory;
            }
            if result >= 10.0 {
                return BattleOutcome::HeroicVictory;
            }
            return BattleOutcome::CloseVictory;
        }
        //Defeat
        if result < -2.0 {
            if result <= -20.0 {
                return BattleOutcome::CrushingDefeat;
            }
            if result <= -10.0 {
                return BattleOutcome::ValiantDefeat;
            }
            return BattleOutcome::CloseDefeat;
        }
        //Draw
        BattleOutcome::Draw

    }
}

// TODO write unit tests for BattleOutcome
// TODO write unit tests for town defenses
// TODO write unit tests for BattleType
// TODO write unit tests for Battle