
extern crate rand;

mod ability;
mod background;
mod character;
mod class;
mod hit_dice;
mod item;
mod language;
mod race;
mod random;
mod skill;

#[derive(Clone, Copy, Debug, PartialEq)]
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
pub use background::Background;
pub use character::Character;
pub use class::{Class, ClassFunctions};
pub use hit_dice::HitDice;
pub use item::armor::ArmorProficiency;
pub use item::weapon::WeaponProficiency;
pub use language::Language;
pub use race::{Race, RaceFunctions, RaceSize};
pub use random::{d6, d8, d10, d12, Selections};
pub use skill::Skill;

pub mod test {
    use super::*;

    pub fn check_saving_throws(
        c: &Character,
        strength: isize, dexterity: isize, constitution: isize,
        intelligence: isize, wisdom: isize, charisma: isize
    ) {
        assert_eq!(c.saving_throw(Ability::Strength), strength);
        assert_eq!(c.saving_throw(Ability::Dexterity), dexterity);
        assert_eq!(c.saving_throw(Ability::Constitution), constitution);
        assert_eq!(c.saving_throw(Ability::Intelligence), intelligence);
        assert_eq!(c.saving_throw(Ability::Wisdom), wisdom);
        assert_eq!(c.saving_throw(Ability::Charisma), charisma);
    }

    pub fn check_skill_modifiers(
        c: &Character,
        acrobatics: isize, animal_handling: isize, arcana: isize,
        athletics: isize, deception: isize, history: isize,
        insight: isize, intimidation: isize, investigation: isize,
        medicine: isize, nature: isize, perception: isize,
        performance: isize, persuasion: isize, religion: isize,
        sleight_of_hand: isize, stealth: isize, survival: isize
    ) {
        assert_eq!(c.skill_check(Skill::Acrobatics), acrobatics);
        assert_eq!(c.skill_check(Skill::AnimalHandling), animal_handling);
        assert_eq!(c.skill_check(Skill::Arcana), arcana);
        assert_eq!(c.skill_check(Skill::Athletics), athletics);
        assert_eq!(c.skill_check(Skill::Deception), deception);
        assert_eq!(c.skill_check(Skill::History), history);
        assert_eq!(c.skill_check(Skill::Insight), insight);
        assert_eq!(c.skill_check(Skill::Intimidation), intimidation);
        assert_eq!(c.skill_check(Skill::Investigation), investigation);
        assert_eq!(c.skill_check(Skill::Medicine), medicine);
        assert_eq!(c.skill_check(Skill::Nature), nature);
        assert_eq!(c.skill_check(Skill::Perception), perception);
        assert_eq!(c.skill_check(Skill::Performance), performance);
        assert_eq!(c.skill_check(Skill::Persuasion), persuasion);
        assert_eq!(c.skill_check(Skill::Religion), religion);
        assert_eq!(c.skill_check(Skill::SleightOfHand), sleight_of_hand);
        assert_eq!(c.skill_check(Skill::Stealth), stealth);
        assert_eq!(c.skill_check(Skill::Survival), survival);
    }
}
