use crate::unit::Unit;
use crate::faction::faction;
use std::fs;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct Roster{
    file_path : String,
    units : Vec<Unit>,
}
impl Roster{
    pub fn new() -> Self {
        let mut r = Roster{
            file_path: String::from("./ResourceFiles/units.csv"),
            units : vec![],
        };
        r.init();
        r
    }

    /// Read in data from file_path and parse into unit objects
    fn init(&mut self) {
        let file = fs::read_to_string(&self.file_path)
            .expect(&format!{"Unable to read {}",self.file_path});

        // Read through lines, skip first as it is the column headers
        for line in file.lines().skip(1){
            self.units.push(self.read_unit(line));
        }
    }

    /// Parse string into Unit object
    /// TODO: update to use CSV package
    pub fn read_unit(&self, line : &str) -> Unit{
        let values : Vec<&str> = line.split(",").collect();
        // println!("{:?}", values);
        Unit::new(
            values[0].trim().parse().unwrap(),
            values[1].trim().parse().unwrap(),
            values[2].trim().parse().unwrap(),
            values[3].trim().parse().unwrap(),
            values[4].trim().parse().unwrap(),
        )
    }

    /// Print all items in units vector
    pub fn print_units(&self){
        println!("Units in roster:");
        for (k,v) in self.units.iter().enumerate(){
            println!("{} : {:?}", k, v)
        }
    }

    /// Get all units of a given faction
    pub fn get_faction_roster(&self, faction: faction) -> Vec<&Unit>{
        self.units.iter().filter(|u| *u.get_faction() == faction).collect()
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_faction_roster(){
        let r = Roster::new();

        for i in r.get_faction_roster(faction::Menoriad).iter(){
            assert_eq!(*i.get_faction(),faction::Menoriad);
        }
        for i in r.get_faction_roster(faction::Beladimir).iter(){
            assert_eq!(*i.get_faction(),faction::Beladimir);
        }
        for i in r.get_faction_roster(faction::Rebel).iter(){
            assert_eq!(*i.get_faction(),faction::Rebel);
        }
        for i in r.get_faction_roster(faction::Lerastir).iter(){
            assert_eq!(*i.get_faction(),faction::Lerastir);
        }

    }

}
