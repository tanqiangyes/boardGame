#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Deserialize, Serialize)]
pub struct Player {
    pub name: &'static str,
    pub age: u8,
    //todo
}

impl Player {
    fn new(name: &'static str, age: u8) -> Self {
        Player {
            name: name,
            age: age,
        }
    }
}
