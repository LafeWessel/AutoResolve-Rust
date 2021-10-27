pub struct Roster{
    placeholder : String,
}
impl Roster{
    pub fn new() -> Roster {
        Roster{
            placeholder: String::from("default")
        }
    }
    pub fn new_and_init() -> Roster{
        let mut r = Roster::new();
        r.init();
        r
    }

    pub fn init(&mut self) {

    }
}