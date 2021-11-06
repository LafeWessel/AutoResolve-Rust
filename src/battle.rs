use crate::treasure::Treasure;
use crate::player::Player;
use crate::monster::MonsterType;
use crate::equipment::{Equipment, EquipmentType};
use crate::general::GeneralState;
use rand::Rng;

struct Battle<'a>{
    battle_type : BattleType,
    attacker : Player,
    defender : Player,
    treasure : &'a Treasure,
}

impl Battle<'_>{
    /// Resolve Battle and return results
    fn autoresolve(&mut self) -> BattleResults{
        let outcome = self.calculate_outcome();
        let mut casualties = self.calculate_casualties(&outcome);
        Self::assign_casualties(&mut casualties.attacker, &mut self.attacker);
        Self::assign_casualties(&mut casualties.defender,&mut self.defender);
        let treasure_results = self.treasure_results();

        BattleResults{
            battle_type : self.battle_type,
            outcome : outcome,
            casualties : casualties,
            treasure: treasure_results,
        }
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
        for _ in 0..att_tot/10{
            att_cas += rng.gen_range(0..((*outcome as i32) + 1));
        }
        let att_unit_cas = if (att_cas/7)-1 < 0 {0} else {(att_cas/7)-1};

        // Defender Casualties
        let def_tot = self.defender.get_soldier_count();
        let mut def_cas = 0;
        for _ in 0..def_tot/10{
            def_cas += rng.gen_range(0..((*outcome as i32) + 1));
        }
        let def_unit_cas = if (def_cas/7)-1 < 0 {0} else {(def_cas/7)-1};

        // Upgrades
        let att_up = def_cas / 6;
        let def_up = att_cas / 6;

        // Attacker General state
        let mut att_gen = GeneralState::Unharmed;
        if rng.gen_range(1..9) <= 2{
            att_gen = GeneralState::Wounded;
            if rng.gen_range(1..9) <= 2{
                att_gen = GeneralState::Slain;
            }
        }
        // Defender General state
        let mut def_gen = GeneralState::Unharmed;
        if rng.gen_range(1..9) <= 2{
            def_gen = GeneralState::Wounded;
            if rng.gen_range(1..9) <= 2{
               def_gen = GeneralState::Slain;
            }
        }

        BattleCasualties {
            attacker: Casualties{
                state: att_gen,
                upgrades: att_up,
                casualties: att_cas,
                unit_casualties: att_unit_cas,
            },
            defender: Casualties{
                state: def_gen,
                upgrades: def_up,
                casualties: def_cas,
                unit_casualties: def_unit_cas,
            },
        }
    }

    /// Assign casualties to a player
    fn assign_casualties(casualties : &mut Casualties, player : &mut Player){
        let mut rng = rand::thread_rng();

        // If casualties > player's soldier count, assign all units to max casualties
        if casualties.casualties > player.get_soldier_count(){
            player.get_units_mut().iter_mut().map(|u| u.assign_casualties(u.get_size())).for_each(drop);
        }

        let mut assigned: i32 = 0; // assigned casualties
        let mut assigned_unit: i32 = 0; // assigned unit casualties
        let mut top_assign : i32 = 0; // top amount of casualties that can be assigned to a unit
        let mut curr_cas:i32 = 0; // current amount of casualties to assign

        // loop through units until all casualties have been assigned
        while assigned < casualties.casualties{
            for u in player.get_units_mut(){
                // skip if unit has no size left
                if u.get_size() <= 0{
                    continue;
                }

                // if all unit casualties assigned, ensure that top_assign is one less than the current unit's size
                top_assign = if assigned_unit >= casualties.unit_casualties {u.get_size()-1} else {u.get_size()};

                // assign random amount of casualties between 0 and top_assign
                curr_cas = rng.gen_range(0..top_assign);

                // prevent more than maximum casualties being assigned
                if assigned + curr_cas > casualties.casualties{
                    curr_cas = casualties.casualties - assigned;
                }
                // assign casualties, ensure it works properly
                assigned += curr_cas;
                assert!(u.assign_casualties(curr_cas));

                // assign unit casualty if unit size = 0
                if u.get_size() == 0{
                    assigned_unit += 1;
                }
            }
        }

        // recalculate number of unit casualties
        let mut tot_u_cas = 0;
        for u in player.get_units_mut().iter(){
            if u.get_size() == 0{
                tot_u_cas += 1;
            }
        }
        casualties.unit_casualties = tot_u_cas;
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
        let bonus = player.get_general().get_equipment(EquipmentType::Follower).get_bonus();
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

struct BattleResults<'a>{
    battle_type : BattleType,
    outcome: BattleOutcome,
    casualties : BattleCasualties,
    treasure : TreasureResults<'a>,
}

impl BattleResults<'_>{
    /// Convert BattleResults to a printable string
    pub fn battle_output(&self) -> String{
        format!("Battle Results:\n\
        Type: {:?}\nOutcome (for Attacker): {:?}\n\
        Attacker Casualties:\n\tSoldiers: {}\n\tUnits: {}\n\tGeneral State: {:?}\n\tUpgrades: {}\n\
        Defender Casualties:\n\tSoldiers: {}\n\tUnits: {}\n\tGeneral State: {:?}\n\tUpgrades: {}\n\
        Attacker Reward: {:?}\nDefender Reward: {:?}",
                            self.battle_type, self.outcome,
                            self.casualties.attacker.casualties,
                            self.casualties.attacker.unit_casualties,
                            self.casualties.attacker.state,
                            self.casualties.attacker.upgrades,
                            self.casualties.defender.casualties,
                            self.casualties.defender.unit_casualties,
                            self.casualties.defender.state,
                            self.casualties.defender.upgrades,
                            self.treasure.defender,self.treasure.attacker)


    }
}

struct BattleCasualties{
    attacker : Casualties,
    defender : Casualties,
}

struct Casualties {
    state : GeneralState,
    upgrades : i32,
    casualties : i32,
    unit_casualties : i32,
}

struct TreasureResults<'a>{
    attacker : Option<&'a Equipment>,
    defender : Option<&'a Equipment>,
}

#[derive(Debug, Copy, Clone)]
enum BattleType {
    Normal,
    Siege{rams: i32, catapults: i32, siege_towers: i32, defenses: TownStats },
    Raid{defenses: TownStats },
    Naval{attacker_ships: i32, defender_ships: i32},
    Monster{monster : MonsterType }
}

impl BattleType {
    /// Calculate the autoresolve modifier for the type of battle
    fn get_calculation(&self) -> i32{
        match &self{
            BattleType::Normal => 0,
            BattleType::Siege { rams,catapults,siege_towers,defenses } => (rams * 2) + (catapults * 3) + (siege_towers * 4) - defenses.get_autoresolve_bonus(),
            BattleType::Raid { defenses } => -1 * defenses.get_autoresolve_bonus(),
            BattleType::Naval { attacker_ships,defender_ships} => 3*(attacker_ships - defender_ships),
            BattleType::Monster { monster} => -1 * monster.autoresolve_value(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

#[cfg(test)]
mod battle_outcome_tests{
    use crate::battle::BattleOutcome;

    #[test]
    fn test_outcome(){
        assert_eq!(BattleOutcome::DecisiveVictory, BattleOutcome::determine_outcome(20.0));
        assert_eq!(BattleOutcome::HeroicVictory, BattleOutcome::determine_outcome(10.0));
        assert_eq!(BattleOutcome::CloseVictory, BattleOutcome::determine_outcome(2.1));
        assert_eq!(BattleOutcome::Draw, BattleOutcome::determine_outcome(2.0));
        assert_eq!(BattleOutcome::Draw, BattleOutcome::determine_outcome(0.0));
        assert_eq!(BattleOutcome::Draw, BattleOutcome::determine_outcome(-2.0));
        assert_eq!(BattleOutcome::CrushingDefeat, BattleOutcome::determine_outcome(-20.0));
        assert_eq!(BattleOutcome::ValiantDefeat, BattleOutcome::determine_outcome(-10.0));
        assert_eq!(BattleOutcome::CloseDefeat, BattleOutcome::determine_outcome(-2.1));
    }
}

#[cfg(test)]
mod battle_type_tests{
    use crate::battle::{BattleType, TownStats, TownDefenses, Battle};
    use crate::monster::MonsterType;

    #[test]
    fn test_battle_type_calculation(){
        // Normal
        assert_eq!(0,BattleType::Normal.get_calculation());

        // Siege
        let ts = TownStats{
            supplies : 0,
            defenses : TownDefenses::None,
        };
        assert_eq!(9,BattleType::Siege {rams:1,siege_towers:1,catapults:1,defenses:ts}.get_calculation());

        // Raid
        let ts = TownStats{
            supplies : 0,
            defenses : TownDefenses::WoodenWall,
        };
        assert_eq!(-10,BattleType::Raid {defenses:ts}.get_calculation());

        // Naval
        assert_eq!(0,BattleType::Naval{attacker_ships:1,defender_ships:1}.get_calculation());

        // Monster
        assert_eq!(-20,BattleType::Monster { monster: MonsterType::Minotaur}.get_calculation())
    }

}

#[cfg(test)]
mod town_stats_tests{
    use crate::battle::TownStats;
    use super::TownDefenses;

    #[test]
    fn test_town_stat_bonus(){
        let mut t = TownStats{
            supplies: 0,
            defenses: TownDefenses::None
        };
        assert_eq!(0,t.get_autoresolve_bonus());


        let mut t = TownStats{
            supplies: 0,
            defenses: TownDefenses::WoodenWall
        };
        assert_eq!(10,t.get_autoresolve_bonus());

        let mut t = TownStats{
            supplies: 0,
            defenses: TownDefenses::WoodenWallAndMoat
        };
        assert_eq!(20,t.get_autoresolve_bonus());

        let mut t = TownStats{
            supplies: 0,
            defenses: TownDefenses::StoneWall
        };
        assert_eq!(30,t.get_autoresolve_bonus());

        let mut t = TownStats{
            supplies: 0,
            defenses: TownDefenses::StoneWallAndMoat
        };
        assert_eq!(40,t.get_autoresolve_bonus());
    }
}

// TODO write unit tests for Battle
#[cfg(test)]
mod battle_tests{

}