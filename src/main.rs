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
    // TODO implement rand
    // TODO implement load battle from JSON
// TODO implement TestBattle: randomized testing of battles
// TODO fix GitHub CI
// TODO add default battle examples in JSON files

fn main() {

    let cfg = Config::initialize();

    cfg.run_app();

}

