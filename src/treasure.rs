use crate::equipment::Equipment;
use std::fs;

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
            let values : Vec<&str> = line.split(",").collect();
            // println!("{:?}", values);
            self.items.push(Equipment::new(
                values[0].trim(),
                values[1].trim().parse().unwrap(),
                values[2].trim().parse().unwrap(),
                values[3].trim().parse().unwrap(),
                values[4].trim().parse().unwrap(),
                values[5].trim().parse().unwrap(),
                values[6].trim().parse().unwrap(),
            ))
        }
    }

    pub fn print_items(&self){
        println!("Items in Treasure:");
        for (k,v) in self.items.iter().enumerate(){
            println!("{} : {:?}", k, v);
        }
    }
}

