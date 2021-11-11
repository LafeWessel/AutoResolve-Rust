use crate::equipment::{Equipment, EquipmentType};
use std::fs;
use rand::seq::SliceRandom;
use rand::Rng;

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
    pub fn get_item(&self, equip_type: EquipmentType) -> &Equipment{
        let v = self.items.iter()
            .filter(|e| *e.equip_type() == equip_type)
            .collect::<Vec<&Equipment>>();
        v.choose(&mut rand::thread_rng()).unwrap()
    }

    /// Get a random equipment that is Dragon
    pub fn get_dragon_equipment(&self) -> &Equipment{
        let v = self.items.iter()
            .filter(|e| e.get_is_dragon())
            .collect::<Vec<&Equipment>>();
        v.choose(&mut rand::thread_rng()).unwrap()
    }

    /// Find equipment for battle results
    pub fn find_equipment(&self) -> &Equipment{
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..6) {
            1 => self.get_item(EquipmentType::Armor),
            2 => self.get_item(EquipmentType::Weapon),
            3 => self.get_item(EquipmentType::Trinket),
            4 => self.get_item(EquipmentType::Banner),
            5 => self.get_item(EquipmentType::Follower),
            i => panic!(format!("RNG {} not in [1..5]!",i)),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_item_test(){
        let t = Treasure::new();

        assert_eq!(EquipmentType::Armor, *t.get_item(EquipmentType::Armor).equip_type());
        assert_eq!(EquipmentType::Weapon, *t.get_item(EquipmentType::Weapon).equip_type());
        assert_eq!(EquipmentType::Trinket, *t.get_item(EquipmentType::Trinket).equip_type());
        assert_eq!(EquipmentType::Banner, *t.get_item(EquipmentType::Banner).equip_type());
        assert_eq!(EquipmentType::Follower, *t.get_item(EquipmentType::Follower).equip_type());
    }

    #[test]
    fn get_dragon_test(){
        let t = Treasure::new();

        assert!(t.get_dragon_equipment().get_is_dragon());
    }
}