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
mod config;

use clap::{App, Arg, ArgMatches};
use crate::battle::{BattleType, TownStats};
use crate::monster::MonsterType;
use crate::config::Config;

// TODO implement CLI argument functionality

fn main() {

    let app  = Config::initialize_clap_app();
    let matches = app.get_matches();
    let cfg = Config::parse_app_arguments(&matches);

    cfg.run_app();

}

