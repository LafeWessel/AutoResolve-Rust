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

use crate::config::Config;

// TODO implement CLI argument functionality
// TODO fix GitHub CI
// TODO add default battle examples in JSON files
// TODO save aggregate data for multiple runs
// TODO convert BattleData field in Battle to be optional?
// TODO create struct solely for serialize/deserializing Battle objects => Remove serialize/deserialize for all objects and re-add as necessary

fn main() {

    let cfg = Config::initialize();

    cfg.run_app();

}

