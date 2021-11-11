use crate::roster::Roster;
use crate::treasure::Treasure;

mod treasure;
mod roster;
mod unit;
mod equipment;
mod faction;
mod general;
mod player;
mod battle;
mod monster;

use clap::{App, Arg, ArgMatches};
use crate::battle::{BattleType, TownStats};
use crate::monster::MonsterType;

fn main() {

    let app  = initialize_clap_app();
    let matches = app.get_matches();
    let cfg = parse_app_arguments(&matches);

    run_app(&cfg);

}

/// Initialize clap App with arguments
fn initialize_clap_app() -> App<'static, 'static>{

    // Arg for running randomly generated battles
    let rand = Arg::with_name("random")
        .short("r").long("random")
        .help("Run randomly generated data");
    // Arg for saving battle runs to file
    let save = Arg::with_name("save")
        .short("s").long("save")
        .help("Save battle runs to file");
    // Arg for specifying which file to save runs to
    let output_file = Arg::with_name("output_file")
        .short("f").long("file")
        .help("Override output file for saving output data")
        .requires("save")
        .value_name("FILE");
    // Arg for specifying how many runs to perform
    let count = Arg::with_name("run_count")
        .short("c").long("count")
        .help("Number of battle runs to perform")
        .value_name("COUNT").default_value("1");
    // Arg for specifying how much logging to output
    let logging = Arg::with_name("log")
        .short("l").long("log")
        .help("Log to console");
    // Arg for specifying which type of battle to run
    let battle_type = Arg::with_name("battle_type")
        .short("b").long("battle")
        .help("Battle type to run. 1:Normal,2:Siege,3:Raid,4:Naval,5:Monster")
        .value_name("TYPE").default_value("1");
    // Arg for specifying a different unit/roster file to use
    let roster_file = Arg::with_name("roster_file")
        .short("u").long("unit")
        .help("Override input file for reading unit/roster data")
        .value_name("FILE");
    // Arg for specifying a different treasure file to use
    let treasure_file = Arg::with_name("treasure_file")
        .short("t").long("treasure")
        .help("Override input file for reading treasure/equipment data")
        .value_name("FILE");

    // Create and return new App
    App::new("Autoresolve")
        .version("1.0.0")
        .author("Lafe Wessel")
        .about("Calculator for \"The Game\"")
        .arg(rand)
        .arg(save)
        .arg(output_file)
        .arg(count)
        .arg(logging)
        .arg(battle_type)
        .arg(roster_file)
        .arg(treasure_file)
}

/// Parse arguments from provided CLI command and return a new Config
fn parse_app_arguments<'a>(matches : &'a ArgMatches) -> Config<'a>{
    Config{
        roster: Roster::new(Option::None),
        treasure: Treasure::new(Option::None),
        use_rand: matches.is_present("random"),
        save_data: matches.is_present("save"),
        output_file_override: matches.value_of("output_file"),
        run_count: matches.value_of("run_count").unwrap().parse().unwrap(),
        log: matches.is_present("log"),
        // use default values for initializing battle type, they can be altered later
        battle_type: match matches.value_of("battle_type").unwrap() {
            "2" => BattleType::Siege { rams: 0, catapults: 0, siege_towers: 0, defenses: TownStats::default(), },
            "3" => BattleType::Raid { defenses: TownStats::default() },
            "4" => BattleType::Naval {attacker_ships:0,defender_ships:0},
            "5" => BattleType::Monster { monster: MonsterType::Minotaur },
            "1" | _ => BattleType::Normal,
        },
        roster_file_override: matches.value_of("roster_file"),
        treasure_file_override: matches.value_of("treasure_file"),
    }
}


/// Run application with provided Config
fn run_app(config : &Config){
    println!("Running application");
}

struct Config<'a> {
    roster : Roster,
    treasure : Treasure,
    use_rand : bool,
    save_data : bool,
    output_file_override : Option<&'a str>,
    run_count: i32,
    log : bool,
    battle_type : BattleType,
    roster_file_override : Option<&'a str>,
    treasure_file_override : Option<&'a str>,

}

// TODO implement TestBattle: randomized testing of battles

// TODO implement CLI using clap crate

// TODO implement logging throughout

#[cfg(test)]
mod cli_tests{
    use crate::{initialize_clap_app, parse_app_arguments};
    use crate::monster::MonsterType;
    use crate::battle::BattleType;

    #[test]
    fn test_default_cli_options(){
        let app = initialize_clap_app();
        let args = vec![""];
        let matches = app.get_matches_from(args);
        let cfg = parse_app_arguments(&matches);
        assert!(!cfg.log);
        assert!(!cfg.save_data);
        assert!(!cfg.use_rand);
        assert_eq!(cfg.run_count, 1);
        assert_eq!(cfg.battle_type,BattleType::Normal);
        assert_eq!(None,cfg.output_file_override);
        assert_eq!(None,cfg.roster_file_override);
        assert_eq!(None,cfg.treasure_file_override);
    }

    #[test]
    fn test_non_default_cli_options(){
        let app = initialize_clap_app();
        let args = vec!["","-r","-s","-f","test1","-c","2","-l","-b","5","-u","test2","-t","test3"];
        let matches = app.get_matches_from(args);
        let cfg = parse_app_arguments(&matches);
        assert!(cfg.log);
        assert!(cfg.save_data);
        assert!(cfg.use_rand);
        assert_eq!(cfg.run_count, 2);
        assert_eq!(cfg.battle_type,BattleType::Monster {monster:MonsterType::Minotaur});
        assert_eq!(Some("test1"),cfg.output_file_override);
        assert_eq!(Some("test2"),cfg.roster_file_override);
        assert_eq!(Some("test3"),cfg.treasure_file_override);
    }
}

