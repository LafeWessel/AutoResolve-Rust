use crate::equipment::{Equipment, EquipmentType};
use std::fs;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone)]
pub struct Treasure{
    file_path : String,
    items : Vec<Equipment>,
}

impl Treasure{
    /// Create new Treasure, defaults to ./ResourceFiles/equipment.csv if None provided
    pub fn new(file_path : Option<&str>) -> Self{
        let mut t = Treasure{
            file_path: match file_path {
                None => String::from("./ResourceFiles/equipment.csv"),
                Some(s) => String::from(s),
            },
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
            values[7].trim().to_lowercase().parse().unwrap(),
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
    pub fn get_item_by_type(&self, equip_type: EquipmentType) -> &Equipment{
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

    /// Get item by id, returns first item if there are multiple with the same id
    pub fn get_item_by_id(&self, id : i32) -> Option<&Equipment>{
        self.items.iter()
            .filter(|e| e.get_id() == id)
            .collect::<Vec<&Equipment>>()
            .get(0).map(|e| e.clone())
    }

    /// Find equipment for battle results
    pub fn battle_find_equipment(&self) -> &Equipment{
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..6) {
            1 => self.get_item_by_type(EquipmentType::Armor),
            2 => self.get_item_by_type(EquipmentType::Weapon),
            3 => self.get_item_by_type(EquipmentType::Trinket),
            4 => self.get_item_by_type(EquipmentType::Banner),
            5 => self.get_item_by_type(EquipmentType::Follower),
            i => panic!("RNG {} not in [1..5]!",i),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_item_test(){
        let t = Treasure::new(Option::None);

        assert_eq!(EquipmentType::Armor, *t.get_item_by_type(EquipmentType::Armor).equip_type());
        assert_eq!(EquipmentType::Weapon, *t.get_item_by_type(EquipmentType::Weapon).equip_type());
        assert_eq!(EquipmentType::Trinket, *t.get_item_by_type(EquipmentType::Trinket).equip_type());
        assert_eq!(EquipmentType::Banner, *t.get_item_by_type(EquipmentType::Banner).equip_type());
        assert_eq!(EquipmentType::Follower, *t.get_item_by_type(EquipmentType::Follower).equip_type());
    }

    #[test]
    fn get_dragon_test(){
        let t = Treasure::new(Option::None);

        assert!(t.get_dragon_equipment().get_is_dragon());
    }

    #[test]
    fn test_get_by_id(){
        let t = Treasure::new(None);
        let e = t.get_item_by_id(0);
        assert_eq!(None, e);
        let e = t.get_item_by_id(1);
        assert!(match e {
            Some(_) => true,
            None => false,
        });
    }
}