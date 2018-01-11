
mod barbarian;
mod bard;
mod cleric;
mod druid;
mod fighter;
mod monk;
mod paladin;
mod ranger;
mod rogue;
mod sorcerer;
mod warlock;
mod wizard;

pub use self::barbarian::Barbarian;
pub use self::bard::Bard;
pub use self::cleric::Cleric;
pub use self::druid::Druid;
pub use self::fighter::Fighter;
pub use self::monk::Monk;
pub use self::paladin::Paladin;
pub use self::ranger::Ranger;
pub use self::rogue::Rogue;
pub use self::sorcerer::Sorcerer;
pub use self::warlock::Warlock;
pub use self::wizard::Wizard;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

// http://engl393-dnd5th.wikia.com/wiki/Classes
#[derive(Clone, Copy, PartialEq)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard
}

impl Class {
    pub fn new(self) -> Box<ClassFunctions> {
        match self {
            Class::Barbarian => Box::new(Barbarian {}),
            Class::Bard => Box::new(Bard {}),
            Class::Cleric => Box::new(Cleric {}),
            Class::Druid => Box::new(Druid {}),
            Class::Fighter => Box::new(Fighter {}),
            Class::Monk => Box::new(Monk {}),
            Class::Paladin => Box::new(Paladin {}),
            Class::Ranger => Box::new(Ranger {}),
            Class::Rogue => Box::new(Rogue {}),
            Class::Sorcerer => Box::new(Sorcerer {}),
            Class::Warlock => Box::new(Warlock {}),
            Class::Wizard => Box::new(Wizard {})
        }
    }
}

pub trait ClassFunctions {
    fn hit_dice(&self) -> HitDice;
    fn armors(&self) -> Vec<ArmorProficiency>;
    fn weapons(&self) -> Vec<WeaponProficiency>;
    // tools
    fn saving_throws(&self) -> [Ability; 2];
    fn skills_choice(&self) -> Selections<Skill>;
    // features

    fn saves_on(&self, ability: Ability) -> bool {
        let st = self.saving_throws();
        st[0] == ability || st[1] == ability
    }

    fn proficiency_points(&self, level: usize) -> usize {
        match level {
            1 | 2 | 3 | 4 => 2,
            5 | 6 | 7 | 8 => 3,
            9 | 10 | 11 | 12 => 4,
            13 | 14 | 15 | 16 => 5,
            17 | 18 | 19 | 20 => 6,
            0 | _ => panic!("Level mut be between 1 and 20")
        }
    }
}
