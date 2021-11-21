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
// TODO print aggregate data for multiple runs
// TODO convert BattleData field in Battle to be optional?
// TODO convert General to have Option<Equipment> instead of Equipment fields
// TODO make data saving for multiple battle runs occur at the end of all runs instead of each run
fn main() {

    let cfg = Config::initialize();

    cfg.run_app();

}

