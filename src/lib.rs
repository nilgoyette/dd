
extern crate rand;

mod ability;
mod armor;
mod background;
mod character;
mod class;
mod hit_dice;
mod race;
mod random;
mod skill;
mod weapon;

#[derive(Clone, PartialEq)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    Neutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil
}

pub use ability::{Ability, Modifier};
pub use armor::{ArmorProficiency};
pub use background::{Background};
pub use character::{Character};
pub use class::Class;
pub use race::{Race};
pub use skill::{Skill};
pub use weapon::{WeaponProficiency};
