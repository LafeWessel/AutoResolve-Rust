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

// TODO fix GitHub CI
// TODO add default battle examples in JSON files
fn main() {

    let cfg = Config::initialize();
    cfg.run_app();

}

