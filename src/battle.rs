use crate::treasure::Treasure;
use crate::player::Player;
use crate::monster::MonsterType;
use crate::equipment::{Equipment, EquipmentType};
use crate::general::GeneralState;
use rand::Rng;
use crate::roster::Roster;
use std::path::Path;
use std::fs::{OpenOptions};
use std::io::Write;
use std::fs;

pub struct Battle<'a>{
    battle_type : BattleType,
    attacker : Player,
    defender : Player,
    treasure : &'a Treasure,
    data : BattleData,
}

impl Battle<'_>{

    pub fn new(attacker : Player, defender: Player, battle_type : BattleType, treasure : &'static Treasure, roster : &Roster, output_file : &Option<String>) -> Self{
        Battle{
            battle_type,
            attacker,
            defender,
            treasure,
            data : BattleData::new(roster, output_file),
        }
    }

    /// Resolve Battle and return results
    fn autoresolve(&mut self) -> BattleResults{
        // determine which calculations to use for battle depending on the type
        match self.battle_type{
            BattleType::Monster { .. } => {
                let outcome = self.monster_outcome();
                let mut casualties = self.calculate_casualties(&outcome);
                Self::assign_casualties(&mut casualties.attacker, &mut self.attacker);
                let treasure_results = self.treasure_results();
                BattleResults{
                    battle_type : self.battle_type,
                    outcome,
                    casualties,
                    treasure: treasure_results,
                }
            },
            _ => {
                let outcome = self.calculate_outcome();
                let mut casualties = self.calculate_casualties(&outcome);
                Self::assign_casualties(&mut casualties.attacker, &mut self.attacker);
                Self::assign_casualties(&mut casualties.defender,&mut self.defender);
                let treasure_results = self.treasure_results();
                BattleResults{
                    battle_type : self.battle_type,
                    outcome,
                    casualties,
                    treasure: treasure_results,
                }
            },
        }
    }

    /// Calculate the outcome for a Monster Battle type
    fn monster_outcome(&mut self) -> BattleOutcome{
        let mut total : f32 = 0.0;
        // add attacker bonus
        total += self.attacker.get_autoresolve_bonus() as f32;

        // add random bonuses
        let att_rand = self.battle_randoms() as f32;
        let def_rand = self.battle_randoms() as f32;
        total += att_rand;
        total -= def_rand;

        // add BattleType bonuses
        total += self.battle_type.get_calculation() as f32;

        // determine outcome
        self.data.collect_battle_calculations(att_rand,def_rand,total);
        BattleOutcome::determine_outcome(total)
    }

    /// Calculate the outcome of the battle based on each Player's statistics
    fn calculate_outcome(&mut self) -> BattleOutcome {
        let mut total : f32 = 0.0;

        // get player autoresolve bonuses
        total += self.attacker.get_autoresolve_bonus() as f32;
        total -= self.defender.get_autoresolve_bonus() as f32;

        // add random bonuses
        let att_rand = self.battle_randoms() as f32;
        let def_rand = self.battle_randoms() as f32;
        total += att_rand;
        total -= def_rand;


        // calculate RPS bonuses
        total += 1.5 * (self.attacker.get_cavalry_bonus() - self.defender.get_ranged_bonus()) as f32;
        total += 1.5 * (self.attacker.get_melee_bonus() - self.defender.get_cavalry_bonus()) as f32;
        total += 1.5 * (self.attacker.get_ranged_bonus() - self.defender.get_melee_bonus()) as f32;

        // add BattleType bonuses
        total += self.battle_type.get_calculation() as f32;

        // determine outcome
        self.data.collect_battle_calculations(att_rand,def_rand,total);
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
        if casualties.casualties >= player.get_soldier_count() || casualties.unit_casualties >= player.get_units().len() as i32{
            player.get_units_mut().iter_mut().map(|u| u.assign_casualties(u.get_size())).for_each(drop);
            return;
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
                curr_cas = rng.gen_range(0..=top_assign);

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

    /// Write Battle data to file
    pub fn save_data(&self){
        self.data.save_to_file();
    }


}

pub struct BattleResults<'a>{
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BattleType {
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


    /// Get file name for where data is saved
    fn get_data_path(&self) -> String{
        match *self{
            BattleType::Normal => String::from("NormalData.csv"),
            BattleType::Siege { .. } => String::from("SiegeData.csv"),
            BattleType::Raid { .. } => String::from("RaidData.csv"),
            BattleType::Naval { .. } => String::from("NavalData.csv"),
            BattleType::Monster { .. } => String::from("MonsterData.csv"),
        }

    }

    /// Get name of enum
    fn get_name(&self) -> String{
        match *self{
            BattleType::Normal => String::from("Normal"),
            BattleType::Siege { .. } => String::from("Siege"),
            BattleType::Raid { .. } => String::from("Raid"),
            BattleType::Naval { .. } => String::from("Naval"),
            BattleType::Monster { .. } => String::from("Monster"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TownStats {
    supplies : i32,
    defenses: TownDefenses,
}

impl Default for TownStats{
    fn default() -> Self {
        TownStats{
            supplies: 0,
            defenses:TownDefenses::None,
        }
    }
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
pub enum TownDefenses {
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

struct BattleData{
    data : Vec<String>,
    unit_names : Vec<String>,
    output_location : String,
    got_initial : bool,
    got_calculations : bool,
    got_results : bool,
}

impl BattleData{
    /// Create new BattleData. If output_file is None, uses default for each kind of BattleType.
    fn new(roster : &Roster, output_file: &Option<String>) -> Self{

        let output = match output_file {
            None => String::from("./DataCapture/"),
            Some(s) => s.clone(),
        };

        BattleData{
            data : vec![String::new();141],
            unit_names : roster.get_all_unit_names(),
            output_location : output,
            got_initial : false,
            got_calculations : false,
            got_results : false,
        }
    }

    /// Save initial battle data before running autoresolve
    fn collect_initial_battle_data(&mut self, battle : &Battle){
        // set output location
        self.output_location.push_str(&*battle.battle_type.get_data_path());

        // Battle type
        self.data[0] = battle.battle_type.get_name();
        // Supplies
        self.data[6] = match battle.battle_type {
            BattleType::Siege { defenses , ..} => defenses.supplies.to_string(),
            BattleType::Raid { defenses,.. } => defenses.supplies.to_string(),
            _ => String::from("0"),
        };

        // Attacker fields
        self.data[7] = battle.attacker.get_general().get_rank().to_string();
        self.data[8] = battle.attacker.get_general().get_bonus().to_string();
        self.data[9] = battle.attacker.get_general().get_equipment(EquipmentType::Armor).get_bonus().to_string();
        self.data[10] = battle.attacker.get_general().get_equipment(EquipmentType::Weapon).get_bonus().to_string();
        self.data[11] = battle.attacker.get_general().get_equipment(EquipmentType::Follower).get_bonus().to_string();
        self.data[12] = battle.attacker.get_general().get_equipment(EquipmentType::Banner).get_bonus().to_string();
        self.data[13] = battle.attacker.get_general().get_equipment(EquipmentType::Trinket).get_bonus().to_string();
        self.data[14] = battle.attacker.has_advanced_combat_deck().to_string();
        self.data[15] = (battle.attacker.get_melee_bonus() + battle.attacker.get_cavalry_bonus() + battle.attacker.get_ranged_bonus()).to_string();
        self.data[16] = battle.attacker.get_melee_bonus().to_string();
        self.data[17] = battle.attacker.get_ranged_bonus().to_string();
        self.data[18] = battle.attacker.get_cavalry_bonus().to_string();
        self.data[19] = battle.attacker.get_soldier_count().to_string();

        // Attacker units
        // This works because the units names are in the same order as the output file
        for (i,j) in self.unit_names.iter().enumerate(){
            self.data[25+i] = battle.attacker.get_unit_count_by_name(j).to_string();
        }

        self.data[63] = battle.attacker.get_units().len().to_string();
        self.data[64] = battle.attacker.get_reinforcements().to_string();

        // Defender fields
        self.data[75] = battle.defender.get_general().get_rank().to_string();
        self.data[76] = battle.defender.get_general().get_bonus().to_string();
        self.data[77] = battle.defender.get_general().get_equipment(EquipmentType::Armor).get_bonus().to_string();
        self.data[78] = battle.defender.get_general().get_equipment(EquipmentType::Weapon).get_bonus().to_string();
        self.data[79] = battle.defender.get_general().get_equipment(EquipmentType::Follower).get_bonus().to_string();
        self.data[80] = battle.defender.get_general().get_equipment(EquipmentType::Banner).get_bonus().to_string();
        self.data[81] = battle.defender.get_general().get_equipment(EquipmentType::Trinket).get_bonus().to_string();
        self.data[82] = battle.defender.has_advanced_combat_deck().to_string();
        self.data[83] = (battle.defender.get_melee_bonus() + battle.defender.get_cavalry_bonus() + battle.defender.get_ranged_bonus()).to_string();
        self.data[84] = battle.defender.get_melee_bonus().to_string();
        self.data[85] = battle.defender.get_ranged_bonus().to_string();
        self.data[86] = battle.defender.get_cavalry_bonus().to_string();
        self.data[87] = battle.defender.get_soldier_count().to_string();

        // Defender units
        // This works because the units names are in the same order as the output file
        for (i,j) in self.unit_names.iter().enumerate(){
            self.data[25+i] = battle.defender.get_unit_count_by_name(j).to_string();
        }

        self.data[131] = battle.defender.get_units().len().to_string();
        self.data[132] = battle.defender.get_reinforcements().to_string();


        self.got_initial = true;
    }

    /// Save battle data after running autoresolve
    fn collect_battle_results(&mut self, results : &BattleResults, battle : &Battle){
        // Outcome
        self.data[4] = format!{"{:?}",results.outcome};
        // Attacker won (bool)
        self.data[5] = if results.outcome as i32 > 3 {false.to_string()} else {true.to_string()};

        // Attacker fields
        self.data[20] = (battle.attacker.get_melee_bonus() + battle.attacker.get_cavalry_bonus() + battle.attacker.get_ranged_bonus()).to_string();
        self.data[21] = battle.attacker.get_melee_bonus().to_string();
        self.data[22] = battle.attacker.get_ranged_bonus().to_string();
        self.data[23] = battle.attacker.get_cavalry_bonus().to_string();
        self.data[24] = battle.attacker.get_soldier_count().to_string();
        self.data[65] = results.casualties.attacker.upgrades.to_string();
        self.data[66] = results.casualties.attacker.unit_casualties.to_string();
        self.data[67] = results.casualties.attacker.casualties.to_string();
        self.data[68] = format!{"{:?}",results.casualties.attacker.state};
        self.data[69] = match results.treasure.attacker {
            None => false.to_string(),
            Some(_) => true.to_string(),
        };
        self.data[70] = format!("{:?}", battle.attacker.get_faction());
        self.data[71] = match battle.battle_type{
            BattleType::Naval {attacker_ships,..} => attacker_ships.to_string(),
            _ => String::from("0"),
        };
        self.data[72] = match battle.battle_type{
            BattleType::Siege {rams,..} => rams.to_string(),
            _ => String::from("0"),
        };
        self.data[73] = match battle.battle_type{
            BattleType::Siege {siege_towers,..} => siege_towers.to_string(),
            _ => String::from("0"),
        };
        self.data[74] = match battle.battle_type{
            BattleType::Siege {catapults,..} => catapults.to_string(),
            _ => String::from("0"),
        };

        // Defender fields
        self.data[88] = (battle.defender.get_melee_bonus() + battle.defender.get_cavalry_bonus() + battle.defender.get_ranged_bonus()).to_string();
        self.data[89] = battle.defender.get_melee_bonus().to_string();
        self.data[90] = battle.defender.get_ranged_bonus().to_string();
        self.data[91] = battle.defender.get_cavalry_bonus().to_string();
        self.data[92] = battle.defender.get_soldier_count().to_string();
        self.data[133] = results.casualties.defender.upgrades.to_string();
        self.data[134] = results.casualties.defender.unit_casualties.to_string();
        self.data[135] = results.casualties.defender.casualties.to_string();
        self.data[136] = format!{"{:?}",results.casualties.defender.state};
        self.data[137] = match results.treasure.defender {
            None => false.to_string(),
            Some(_) => true.to_string(),
        };
        self.data[138] = format!("{:?}", battle.defender.get_faction());
        self.data[139] = match battle.battle_type{
            BattleType::Naval {defender_ships,..} => defender_ships.to_string(),
            _ => String::from("0"),
        };
        self.data[140] = match battle.battle_type{
            BattleType::Siege {defenses,..} => format!("{:?}",defenses.defenses),
            BattleType::Raid {defenses,..} => format!("{:?}",defenses.defenses),
            _ => String::from("0"),
        };

        self.got_results = true;
    }


    /// Save calculations made while calculating a battle's outcome
    fn collect_battle_calculations(&mut self, attacker : f32, defender : f32, total: f32){
        // Attacker random total
        self.data[1] = attacker.to_string();
        // Defender random total
        self.data[2] = defender.to_string();
        // Ending total
        self.data[3] = total.to_string();
        self.got_calculations = true;
    }

    /// Save results to disk, return if operation was successful
    fn save_to_file(&self) -> bool{
        if !self.got_calculations || !self.got_results || !self.got_initial{
            println!("Unable to write because not all data yet set\n\t\
            Initial:{}\n\tRandoms:{}\n\tResults:{}"
                     ,self.got_initial,self.got_calculations,self.got_results);
            return false;
        }

        let file_path = Path::new(&self.output_location);
        // If output file doesn't exist, create by copying template
        if !Path::exists(file_path){
            println!("Creating output file at {} for battle data",self.output_location);
            fs::copy("./ResourceFiles/data_capture_template.txt", &self.output_location).unwrap();
        }

        // Write lines to file
        let mut f = OpenOptions::new().write(true).append(true).open(file_path).unwrap();
        // Write each data entry, write the first before to ensure proper comma alignment
        write!(f,"{}",self.data[0]);
        for line in self.data.iter().skip(1){
            // Write each cell, separating with commas
            write!(f,",{}",line);
        }
        write!(f,"\n");
        true
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


    #[test]
    fn test_outcome_to_i32(){
        assert_eq!(1,BattleOutcome::DecisiveVictory as i32);
        assert_eq!(2,BattleOutcome::HeroicVictory as i32);
        assert_eq!(3,BattleOutcome::CloseVictory as i32);
        assert_eq!(4,BattleOutcome::Draw as i32);
        assert_eq!(5,BattleOutcome::CloseDefeat as i32);
        assert_eq!(6,BattleOutcome::ValiantDefeat as i32);
        assert_eq!(7,BattleOutcome::CrushingDefeat as i32);

    }
}

#[cfg(test)]
mod battle_type_tests{
    use crate::battle::{BattleType, TownStats, TownDefenses};
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
        let t = TownStats{
            supplies: 0,
            defenses: TownDefenses::None
        };
        assert_eq!(0,t.get_autoresolve_bonus());


        let t = TownStats{
            supplies: 0,
            defenses: TownDefenses::WoodenWall
        };
        assert_eq!(10,t.get_autoresolve_bonus());

        let t = TownStats{
            supplies: 0,
            defenses: TownDefenses::WoodenWallAndMoat
        };
        assert_eq!(20,t.get_autoresolve_bonus());

        let t = TownStats{
            supplies: 0,
            defenses: TownDefenses::StoneWall
        };
        assert_eq!(30,t.get_autoresolve_bonus());

        let t = TownStats{
            supplies: 0,
            defenses: TownDefenses::StoneWallAndMoat
        };
        assert_eq!(40,t.get_autoresolve_bonus());
    }
}

#[cfg(test)]
mod battle_tests{
    use crate::player::Player;
    use crate::unit::Unit;
    use crate::general::{General, GeneralState};
    use crate::battle::{Casualties, Battle};

    // assign_casualties
    #[test]
    fn test_assign_casualties_equal_casualties() {
        let u = Unit::new(1, String::new(), 1, 1, 5);
        let g = General::default();
        let mut p = Player::new(vec![u], g);
        let mut c = Casualties {
            state: GeneralState::Unharmed,
            upgrades: 0,
            casualties: 5,
            unit_casualties: 0
        };

        Battle::assign_casualties(&mut c, &mut p);
        assert_eq!(0, p.get_units_mut().iter().nth(0).unwrap().get_size());
        assert_eq!(0, p.get_soldier_count());
    }

    #[test]
    fn test_assign_casualties_one_less_casualty(){
        let u = Unit::new(1, String::new(), 1, 1, 5);
        let g = General::default();
        let mut p = Player::new(vec![u], g);
        let mut c = Casualties {
            state: GeneralState::Unharmed,
            upgrades: 0,
            casualties: 4,
            unit_casualties: 0
        };
        Battle::assign_casualties(&mut c, &mut p);
        assert_eq!(1, p.get_units_mut().iter().nth(0).unwrap().get_size());
        assert_eq!(1,p.get_soldier_count());
    }

    #[test]
    fn test_assign_casualties_equal_unit_casualties(){
        let u = Unit::new(1, String::new(), 1, 0, 5);
        let g = General::default();
        let mut p = Player::new(vec![u], g);
        let mut c = Casualties {
            state: GeneralState::Unharmed,
            upgrades: 0,
            casualties: 0,
            unit_casualties: 1
        };
        Battle::assign_casualties(&mut c, &mut p);
        assert_eq!(0, p.get_units_mut().iter().nth(0).unwrap().get_size());
        assert_eq!(0,p.get_soldier_count());
    }

    #[test]
    fn test_assign_casualties_one_less_unit_casualty(){
        let u = Unit::new(1, String::new(), 1, 0, 5);
        let g = General::default();
        let mut p = Player::new(vec![u.clone(),u.clone()], g);
        let mut c = Casualties {
            state: GeneralState::Unharmed,
            upgrades: 0,
            casualties: 9,
            unit_casualties: 1
        };
        Battle::assign_casualties(&mut c, &mut p);

        assert!(0 == p.get_units_mut().iter().nth(0).unwrap().get_size() || 0 == p.get_units_mut().iter().nth(1).unwrap().get_size());
        assert_eq!(1,p.get_soldier_count());
    }

}

#[cfg(test)]
mod battle_data_tests{
    use crate::battle::BattleData;
    use std::path::Path;
    use std::fs;

    #[test]
    fn test_write_to_file(){
        let mut b = BattleData{
            data: vec![String::new()],
            unit_names: vec![],
            output_location: "./DataCapture/test.csv".to_string(),
            got_initial: false,
            got_calculations: false,
            got_results: false
        };
        // remove output file if it somehow exists
        fs::remove_file(Path::new(&b.output_location));

        assert_eq!(false, b.save_to_file());
        b.got_results = true;

        assert_eq!(false, b.save_to_file());
        b.got_calculations = true;

        assert_eq!(false, b.save_to_file());
        b.got_initial = true;

        assert_eq!(false,Path::exists(Path::new("./DataCapture/test.csv")));
        assert_eq!(true, b.save_to_file());
        assert_eq!(true,Path::exists(Path::new("./DataCapture/test.csv")));

        let t : String = fs::read_to_string("./ResourceFiles/data_capture_template.txt").unwrap().trim().parse().unwrap();
        let f : String = fs::read_to_string(&b.output_location).unwrap().trim().parse().unwrap();
        assert_eq!(t,f);

        b.data = vec![String::from("0"),String::from("1"),String::from("2")];
        fs::remove_file(Path::new(&b.output_location));
        assert_eq!(true,b.save_to_file());
        let f : String = fs::read_to_string(&b.output_location).unwrap();
        assert_eq!(f.lines().nth(1).unwrap().trim().parse::<String>().unwrap(), "0,1,2");

        assert_eq!(true,b.save_to_file());
        let f : String = fs::read_to_string(&b.output_location).unwrap();
        assert_eq!(f.lines().nth(1).unwrap().trim().parse::<String>().unwrap(), "0,1,2");
        assert_eq!(f.lines().nth(2).unwrap().trim().parse::<String>().unwrap(), "0,1,2");

        // clean up
        fs::remove_file(Path::new(&b.output_location));
    }
}