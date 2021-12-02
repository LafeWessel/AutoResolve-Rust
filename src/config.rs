
use clap::{App, Arg, ArgMatches};
use crate::battle::{BattleType, TownStats, Battle, BattleJSONObject, BattleOutcome, BattleData, BattleResults};
use crate::monster::MonsterType;
use crate::roster::Roster;
use crate::treasure::Treasure;
use crate::player::Player;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;
use std::sync::mpsc::{channel, Sender, Receiver};
use threadpool::ThreadPool;
use std::sync::{Mutex, Arc};


pub struct Config {
    roster : Roster,
    treasure : Treasure,
    use_rand : bool,
    save_data : bool,
    log : bool,
    output_file_override : Option<String>,
    run_count: u32,
    battle_type : Option<BattleType>,
    battle_file : Option<String>,
    multithread : bool

}

impl Config{
    /// Create new Config based on CLI args
    pub fn initialize() -> Config{
        let app  = Self::initialize_clap_app();
        let matches = app.get_matches();
        let cfg = Self::parse_app_arguments(&matches);

        cfg
    }

    /// Run application with provided Config
    pub fn run_app(&self){

        // aggregate data for runs
        let mut battle_outcomes : [i32;7] = [0;7];

        // Use Normal battle if none specified
        let mut b_type= self.battle_type.unwrap_or_else(|| BattleType::Normal);

        // create Battle
        let b = match &self.battle_file {
            // using a JSON battle
            Some(s) => {
                let b = BattleJSONObject::from_json(s).produce_battle(&self.roster, &self.treasure);
                b_type = b.get_battle_type();
                b
            },
            // not using JSON
            None => Battle::new(Player::default(), Player::default(), b_type)
        };

        // run battles with either one or multiple threads
        let data : (Vec<BattleData>, Vec<BattleResults>) = match self.multithread{
            true => self.run_multiple_threads(&b, self.run_count),
            false => Config::run_single_thread(&b, self.run_count, &self.roster, &self.treasure, self.use_rand, self.battle_type)
        };

        // output data for each battle
        if self.log{
            data.1.iter().for_each(|r| println!("{}",r.battle_output()));
        }

        // save outcome data
        data.0.iter().for_each(|d|
            match d.get_outcome(){
                BattleOutcome::DecisiveVictory => battle_outcomes[0] += 1,
                BattleOutcome::HeroicVictory => battle_outcomes[1] += 1,
                BattleOutcome::CloseVictory => battle_outcomes[2] += 1,
                BattleOutcome::Draw => battle_outcomes[3] += 1,
                BattleOutcome::CloseDefeat => battle_outcomes[4] += 1,
                BattleOutcome::ValiantDefeat => battle_outcomes[5] += 1,
                BattleOutcome::CrushingDefeat => battle_outcomes[6] += 1,
            }
        );

        // print general result statistics
        println!("Battle Type: {}\nResults(For attacker):\n\
        Decisive Victory:{}\n\
        Heroic Victory:{}\n\
        Close Victory:{}\n\
        Draw:{}\n\
        Close Defeat:{}\n\
        Valiant Defeat:{}\n\
        Crushing Defeat:{}",
                 if (self.use_rand) && (self.battle_type == None) {String::from("Random")} else {b_type.get_name()},
                 battle_outcomes[0], battle_outcomes[1], battle_outcomes[2],
                 battle_outcomes[3],
                 battle_outcomes[4], battle_outcomes[5], battle_outcomes[6]);

        // save data to file
        if self.save_data {
            self.save_run_results(&data.0, b_type)
        }
    }

    /// Run all calculations using a single thread
    fn run_single_thread(battle: &Battle, count : u32, roster : &Roster, treasure : &Treasure,  use_rand : bool, battle_type : Option<BattleType>) -> (Vec<BattleData>, Vec<BattleResults>) {
        let mut data : Vec<BattleData> = vec![];
        let mut res : Vec<BattleResults> = vec![];

        for i in 1..=count {
            // run battles

            // create temp battle
            let mut temp = match use_rand{
                true => {
                    Battle::generate_random_battle(roster,treasure,3,10,5, battle_type)
                }
                false => {
                    battle.clone()
                }
            };

            let r = Config::autoresolve_battle(&mut temp, roster, treasure);
            data.push(r.0);
            res.push(r.1);
        }
        (data, res)
    }

    /// Run calculations utilizing multiple threads
    fn run_multiple_threads<'a>(&self, battle : &'a Battle, count : u32) -> (Vec<BattleData>, Vec<BattleResults>){
        let mut data : Vec<BattleData> = vec![];
        let mut res : Vec<BattleResults> = vec![];

        // create multiple producer, single consumer channel for receiving results tuples
        let (tx,rx) : (Sender<(Vec<BattleData>, Vec<BattleResults>)>, Receiver<(Vec<BattleData>, Vec<BattleResults>)>)= channel();

        // determine how many threads to create
        let num_threads = num_cpus::get();

        // create ThreadPool
        let pool = ThreadPool::new(num_threads);

        // determine how to break up self.run_count to run all calculations
        let ct_per_thread: u32 = count / (num_threads as u32 - 1);
        let remainder = count % (num_threads as u32 - 1);

        // run and receive data from threads
        for i in 0..(num_threads - 1){
            let tx_c = tx.clone();
            let ros = self.roster.clone();
            let tr = self.treasure.clone();
            let bat = battle.clone();
            let rand = self.use_rand;
            let b_type = self.battle_type.clone();
            pool.execute(move || {
                let r = ros;
                let t = tr;
                let b = bat;
                let thread_results = Config::run_single_thread(&b, ct_per_thread, &r, &t, rand, b_type);
                tx_c.send(thread_results).expect("Unable to send results through tx channel");
            });
        }

        // run remainder calculations
        let tx_c = tx.clone();
        let ros = self.roster.clone();
        let tr = self.treasure.clone();
        let bat = battle.clone();
        let rand = self.use_rand;
        let b_type = self.battle_type.clone();
        pool.execute(move || {
            let r = ros;
            let t = tr;
            let b = bat;
            let thread_results = Config::run_single_thread(&b, remainder, &r, &t, rand, b_type);
            tx_c.send(thread_results).expect("Unable to send results through tx channel");
        });

        // ensure all threads have completed before continuing
        pool.join();
        assert_eq!(0,pool.panic_count());

        // save results to vectors
        for r in rx{
            r.0.iter().map(|d| data.push(d.clone())).for_each(drop);
            r.1.iter().map(|r| res.push(r.clone())).for_each(drop);
        }

        (data, res)
    }

    /// Autoresolve a single battle and return the BattleData and BattleResults structs
    fn autoresolve_battle(battle: &mut Battle, roster : &Roster, treasure : &Treasure) -> (BattleData, BattleResults){
        let mut data = BattleData::new(roster);
        let res = battle.autoresolve(treasure, &mut data);
        (data,res)
    }

    /// Save set of run results to file
    fn save_run_results(&self, data: &Vec<BattleData>, b_type: BattleType) {
        // create BufWriter and write to file

        // Determine what the output file should be
        let output_file: String = match &self.output_file_override {
            // use default
            None => String::from(format!("./DataCapture/{}", b_type.get_data_path())),
            // override default
            Some(s) => s.clone()
        };
        print!("Saving results to file {}...", output_file);

        let file_path = Path::new(&output_file);
        // If output file doesn't exist, create by copying template
        if !Path::exists(file_path) {
            println!("\nCreated output file at {} for battle data", output_file);
            fs::copy("./ResourceFiles/data_capture_template.txt", &output_file).unwrap();
        }

        // Open output file
        let f = OpenOptions::new().write(true).append(true).open(file_path).unwrap();

        // Write lines to file
        let mut writer = BufWriter::new(f);
        data.iter().for_each(|d| writeln!(writer, "{}", d.format_output()).unwrap());
        println!("Done");
    }

    /// Parse arguments from provided CLI command and return a new Config
    fn parse_app_arguments(matches : &ArgMatches) -> Config{
        Config{
            roster: Roster::new(matches.value_of("roster_file")),
            treasure: Treasure::new(matches.value_of("treasure_file")),
            use_rand: matches.is_present("random"),
            save_data: matches.is_present("save"),
            log : matches.is_present("log"),
            output_file_override: matches.value_of("output_file").map(|s| s.to_string()),
            run_count: matches.value_of("run_count").unwrap().parse().unwrap(),
            // use default values for initializing battle type, they can be altered later
            battle_type: matches.value_of("battle_type").map( |s| match s {
                "2" => BattleType::Siege { rams: 0, catapults: 0, siege_towers: 0, defenses: TownStats::default(), },
                "3" => BattleType::Raid { defenses: TownStats::default() },
                "4" => BattleType::Naval {attacker_ships:0,defender_ships:0},
                "5" => BattleType::Monster { monster: MonsterType::Minotaur },
                "1" | _ => BattleType::Normal,
            }),
            battle_file: matches.value_of("battle_file").map(|s| s.to_string()),
            multithread: matches.is_present("multithread"),
        }
    }

    /// Initialize clap App with arguments
    fn initialize_clap_app() -> App<'static, 'static>{

        // Arg for running randomly generated battles
        let rand = Arg::with_name("random")
            .short("r").long("random")
            .help("Run randomly generated data")
            .conflicts_with("battle_file");
        // Arg for saving battle runs to file
        let save = Arg::with_name("save")
            .short("s").long("save")
            .help("Save battle runs to file");
        let log = Arg::with_name("log")
            .short("l").long("log")
            .help("Print results from each battle run");
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
        // Arg for specifying which type of battle to run
        let battle_type = Arg::with_name("battle_type")
            .short("b").long("battle")
            .help("Battle type to run. 1:Normal,2:Siege,3:Raid,4:Naval,5:Monster")
            .value_name("TYPE")
            .conflicts_with("battle_file");
        // Arg for specifying a different unit/roster file to use
        let roster_file = Arg::with_name("roster_file")
            .long("unit")
            .help("Override input file for reading unit/roster data")
            .value_name("FILE");
        // Arg for specifying a different treasure file to use
        let treasure_file = Arg::with_name("treasure_file")
            .long("treasure")
            .help("Override input file for reading treasure/equipment data")
            .value_name("FILE");
        // Arg for specifying situation file to run
        let battle_file = Arg::with_name("battle_file")
            .short("j").long("json")
            .help("Battle JSON file to read and run.")
            .value_name("FILE")
            .conflicts_with_all(&["random","battle_type"]);
        let multithread = Arg::with_name("multithread")
            .short("m").long("multithread")
            .help("Utilize multiple threads for running calculations");

        // Create and return new App
        App::new("Autoresolve")
            .version("1.0.0")
            .author("Lafe Wessel")
            .about("Calculator for \"The Game\"")
            .arg(rand)
            .arg(save)
            .arg(output_file)
            .arg(count)
            .arg(battle_type)
            .arg(roster_file)
            .arg(treasure_file)
            .arg(battle_file)
            .arg(log)
            .arg(multithread)
    }

}

#[cfg(test)]
mod cli_tests{
    use super::*;
    use crate::monster::MonsterType;
    use crate::battle::BattleType;

    #[test]
    fn test_default_cli_options(){
        let app = Config::initialize_clap_app();
        let args = vec![""];
        let matches = app.get_matches_from(args);
        let cfg = Config::parse_app_arguments(&matches);
        assert!(!cfg.save_data);
        assert!(!cfg.use_rand);
        assert!(!cfg.log);
        assert_eq!(cfg.run_count, 1);
        assert_eq!(cfg.battle_type,None);
        assert_eq!(None,cfg.output_file_override);
        assert_eq!(None,cfg.battle_file);
        assert!(!cfg.multithread);
    }

    #[test]
    fn test_non_default_cli_options(){
        let app = Config::initialize_clap_app();
        let args = vec!["","-r","-s","-f","test1","-c","2","-m","-b","5","--unit","./ResourceFiles/units.csv","--treasure","./ResourceFiles/equipment.csv","-l"];
        let matches = app.get_matches_from(args);
        let cfg = Config::parse_app_arguments(&matches);
        assert!(cfg.save_data);
        assert!(cfg.use_rand);
        assert!(cfg.log);
        assert_eq!(cfg.run_count, 2);
        assert_eq!(cfg.battle_type,Some(BattleType::Monster {monster:MonsterType::Minotaur}));
        assert_eq!(Some("test1".to_string()),cfg.output_file_override);
        assert_eq!(None,cfg.battle_file);
        assert!(cfg.multithread);
    }

    #[test]
    fn test_json_template(){
        let app = Config::initialize_clap_app();
        let args = vec!["","--json","./ResourceFiles/normal_battle_template.json"];
        let matches = app.get_matches_from(args);
        let cfg = Config::parse_app_arguments(&matches);
        assert_eq!(Some("./ResourceFiles/normal_battle_template.json".to_string()),cfg.battle_file);
    }

}

