use crate::equipment::{Equipment, equipment_type};
use std::fs;
use rand::seq::SliceRandom;

pub struct Treasure{
    file_path : String,
    items : Vec<Equipment>,

}

impl Treasure{
    pub fn new() -> Self{
        let mut t = Treasure{
            file_path: String::from("./ResourceFiles/equipment.csv"),
            items : vec![],
        };
        t.init();
        t
    }


    /// Read in data from file_path and parse into new Equipment objects
    fn init(&mut self){
        let file = fs::read_to_string(&self.file_path)
            .expect(&format!{"Unable to read {}",self.file_path});

        // Read through lines, skip first as it is the column headers
        for line in file.lines().skip(1){
            self.items.push(self.read_equipment(line));
        }
    }

    /// Parse string into new Equipment object
    /// TODO: Update to use CSV package
    fn read_equipment(&self, line: &str) -> Equipment{
        let values : Vec<&str> = line.split(",").collect();
        Equipment::new(
            values[0].trim(),
            values[1].trim().parse().unwrap(),
            values[2].trim().parse().unwrap(),
            values[3].trim().parse().unwrap(),
            values[4].trim().parse().unwrap(),
            values[5].trim().parse().unwrap(),
            values[6].trim().parse().unwrap(),
            values[7].trim().parse().unwrap(),
        )
    }

    /// Print all items in items vector
    pub fn print_items(&self){
        println!("Items in Treasure:");
        for (k,v) in self.items.iter().enumerate(){
            println!("{} : {:?}", k, v);
        }
    }

    /// Get random equipment of equip_type
    pub fn get_item(&self, equip_type: equipment_type) -> &Equipment{
        let v = self.items.iter()
            .filter(|e| *e.equip_type() == equip_type)
            .collect::<Vec<&Equipment>>();
        v.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_item_test(){
        let t = Treasure::new();

        assert_eq!(equipment_type::Armor,*t.get_item(equipment_type::Armor).equip_type());
        assert_eq!(equipment_type::Weapon,*t.get_item(equipment_type::Weapon).equip_type());
        assert_eq!(equipment_type::Trinket,*t.get_item(equipment_type::Trinket).equip_type());
        assert_eq!(equipment_type::Banner,*t.get_item(equipment_type::Banner).equip_type());
        assert_eq!(equipment_type::Follower,*t.get_item(equipment_type::Follower).equip_type());
    }
}