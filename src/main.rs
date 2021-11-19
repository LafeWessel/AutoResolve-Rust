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
    // TODO implement log
    // TODO implement rand
    // TODO implement save
    // TODO implement save file override
    // TODO implement load battle from JSON
    // TODO implement battle type
// TODO implement TestBattle: randomized testing of battles

fn main() {

    let cfg = Config::initialize();

    cfg.run_app();

}

