pub struct Treasure{
    placeholder : String,
}

impl Treasure{
    pub fn new() -> Self{
        Treasure{
            placeholder: String::from("default")
        }
    }

    pub fn new_and_init() -> Self{
        let mut t  = Treasure::new();
        t.init();
        t
    }

    pub fn init(&mut self){

    }
}

