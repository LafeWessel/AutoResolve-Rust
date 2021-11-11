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

use clap::{App, Arg};

fn main() {

    let app  = initialize_clap_app();
    let matches = app.get_matches();

    let cfg = Config::new();
}

fn initialize_clap_app() -> App<'static, 'static>{

    // Arg for running randomly generated battles
    let rand = Arg::with_name("random").short("-r").long("--random").help("Run randomly generated data");
    // Arg for saving battle runs to file
    let save = Arg::with_name("save").short("-s").long("--save").help("Save battle runs to file");
    // Arg for specifying which file to save runs to
    let output_file = Arg::with_name("output_file").short("-f").long("--file").help("Override output file for saving output data");
    // Arg for specifying how many runs to perform
    let count = Arg::with_name("run_count").short("-c").long("--count").help("Number of battle runs to perform");
    // Arg for specifying how much logging to output
    let logging = Arg::with_name("log").short("-l").long("--log").help("Log to console");
    // Arg for specifying which type of battle to run
    let battle_type = Arg::with_name("battle_type").short("-b").long("--battle").help("Battle type to run");
    // Arg for specifying a different unit/roster file to use
    let roster_file = Arg::with_name("roster_file").short("-u").long("--unit").help("Override input file for reading unit/roster data");
    // Arg for specifying a different treasure file to use
    let treasure_file = Arg::with_name("treasure_file").short("-t").long("--treasure").help("Override input file for reading treasure/equipment data");


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







struct Config {
    roster : Roster,
    treasure : Treasure,
}

impl Config {
    pub fn new() -> Self{
        Config {
            roster : Roster::new(),
            treasure : Treasure::new(),
        }
    }
}

// TODO implement TestBattle: randomized testing of battles

// TODO implement CLI using clap crate

// TODO implement logging throughout
