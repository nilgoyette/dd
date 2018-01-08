
#[derive(Clone, PartialEq)]
pub enum ArmorProficiency {
    Light,
    Medium,
    Heavy,
    Shield
}

impl ArmorProficiency {
    pub fn all() -> Vec<ArmorProficiency> {
        vec![ArmorProficiency::Light, ArmorProficiency::Medium,
             ArmorProficiency::Heavy, ArmorProficiency::Shield]
    }
}
