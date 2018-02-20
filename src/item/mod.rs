
pub mod armor;
pub mod item;
pub mod weapon;

pub use self::armor::{Armor, ArmorProficiency};
pub use self::weapon::WeaponProficiency;

// https://roll20.net/compendium/dnd5e/Armor#content
pub fn all_armors() -> Vec<Armor> {
    vec![
        Armor::light("Padded", 8.0, 5, 11, true),
        Armor::light("Leather", 10.0, 10, 11, false),
        Armor::light("Studded Leather", 13.0, 45, 12, false),
        Armor::medium("Hide", 12.0, 10, 12, false),
        Armor::medium("Chain Shirt", 20.0, 50, 13, false),
        Armor::medium("Scale Mail", 45.0, 50, 14, true),
        Armor::medium("Breastplate", 20.0, 400, 14, false),
        Armor::medium("Half Plate", 40.0, 750, 15, true),
        Armor::heavy("Ring Mail", 40.0, 30, 14, None),
        Armor::heavy("Chain Mail", 55.0, 75, 16, Some(13)),
        Armor::heavy("Splint", 60.0, 200, 17, Some(15)),
        Armor::heavy("Plate", 65.0, 1500, 18, Some(15))]
}
