
pub struct Item {
    pub name: String,
    pub weight: f32,
    pub cost: usize,
    pub cursed: bool,
    //pub effects: Vec<Effect>
}

impl Item {
    pub fn new(
        name: &str,
        weight: f32,
        cost: usize
    ) -> Item {
        Item { name: name.to_string(), weight, cost, cursed: false }
    }
}
