use crate::unit::Unit;
use crate::faction::Faction;
use std::fs;

#[derive(Debug)]
pub struct Roster{
    file_path : String,
    units : Vec<Unit>,
}
impl Roster{
    /// Create new Roster, defaults to ./ResourceFiles/units.csv if None provided
    pub fn new(file_path : Option<&str>) -> Self {
        let mut r = Roster{
            file_path: match file_path{
                None => String::from("./ResourceFiles/units.csv"),
                Some(s) => String::from(s),
            },
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

    /// Get all units of a given Faction
    pub fn get_faction_roster(&self, faction: Faction) -> Vec<&Unit>{
        self.units.iter().filter(|u| *u.get_faction() == faction).collect()
    }

    /// Return names of every unit
    pub fn get_all_unit_names(&self) -> Vec<String>{
        let mut v : Vec<String> = vec![];
        self.units.iter().map(|u| v.push(String::from(u.get_name()))).for_each(drop);
        v
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_faction_roster(){
        let r = Roster::new(Option::None);

        for i in r.get_faction_roster(Faction::Menoriad).iter(){
            assert_eq!(*i.get_faction(), Faction::Menoriad);
        }
        for i in r.get_faction_roster(Faction::Beladimir).iter(){
            assert_eq!(*i.get_faction(), Faction::Beladimir);
        }
        for i in r.get_faction_roster(Faction::Rebel).iter(){
            assert_eq!(*i.get_faction(), Faction::Rebel);
        }
        for i in r.get_faction_roster(Faction::Lerastir).iter(){
            assert_eq!(*i.get_faction(), Faction::Lerastir);
        }

    }

}
