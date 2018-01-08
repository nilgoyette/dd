
use ability::{Ability};
use armor::ArmorProficiency;
use hit_dice::HitDice;
use random::Choice;
use skill::Skill;
use weapon::WeaponProficiency;

// http://engl393-dnd5th.wikia.com/wiki/Classes
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
    pub fn hit_dice(&self) -> HitDice {
        match *self {
            Class::Barbarian
                => HitDice::D12,
            Class::Fighter | Class::Paladin | Class::Ranger
                => HitDice::D10,
            Class::Bard | Class::Cleric | Class::Druid |
            Class::Monk | Class::Rogue
                => HitDice::D8,
            Class::Sorcerer | Class::Warlock | Class::Wizard
                => HitDice::D6
        }
    }

    pub fn armors(&self) -> Vec<ArmorProficiency> {
        match *self {
            Class::Fighter | Class::Paladin
                => ArmorProficiency::all(),
            Class::Barbarian | Class::Cleric | Class::Druid | Class::Ranger
                => vec![ArmorProficiency::Light,
                        ArmorProficiency::Medium,
                        ArmorProficiency::Shield],
            Class::Bard | Class::Rogue | Class::Warlock
                => vec![ArmorProficiency::Light],
            Class::Monk | Class::Sorcerer | Class::Wizard
                => vec![]
        }
    }

    pub fn weapons(&self) -> Vec<WeaponProficiency> {
        match *self {
            Class::Barbarian | Class::Fighter | Class::Paladin | Class::Ranger
                => WeaponProficiency::all(),
            Class::Bard | Class::Rogue => {
                let mut weapons = Vec::with_capacity(19);
                weapons.extend(WeaponProficiency::simple());
                weapons.extend(vec![
                    WeaponProficiency::HandCrossbow,
                    WeaponProficiency::LongSword,
                    WeaponProficiency::Rapier,
                    WeaponProficiency::ShortSword]);
                weapons
            },
            Class::Cleric | Class::Warlock
                => WeaponProficiency::simple(),
            Class::Monk=> {
                let mut weapons = Vec::with_capacity(16);
                weapons.extend(WeaponProficiency::simple());
                weapons.push(WeaponProficiency::ShortSword);
                weapons
            },
            Class::Druid
                => vec![
                    WeaponProficiency::Club, WeaponProficiency::Dagger,
                    WeaponProficiency::Dart, WeaponProficiency::Javelin,
                    WeaponProficiency::Mace, WeaponProficiency::QuarterStaff,
                    WeaponProficiency::Scimitar, WeaponProficiency::Sickle,
                    WeaponProficiency::Sling, WeaponProficiency::Spear],
            Class::Sorcerer | Class::Wizard
                => vec![
                    WeaponProficiency::Dagger, WeaponProficiency::Dart,
                    WeaponProficiency::Sling, WeaponProficiency::QuarterStaff,
                    WeaponProficiency::LightCrossbow],
        }
    }

    // tools

    pub fn saving_throws(&self) -> [Ability; 2] {
        match *self {
            Class::Barbarian => [Ability::Constitution, Ability::Strength],
            Class::Bard => [Ability::Charisma, Ability::Dexterity],
            Class::Cleric => [Ability::Charisma, Ability::Wisdom],
            Class::Druid => [Ability::Intelligence, Ability::Wisdom],
            Class::Fighter => [Ability::Constitution, Ability::Strength],
            Class::Monk => [Ability::Dexterity, Ability::Strength],
            Class::Paladin => [Ability::Charisma, Ability::Wisdom],
            Class::Ranger => [Ability::Dexterity, Ability::Strength],
            Class::Rogue => [Ability::Dexterity, Ability::Intelligence],
            Class::Sorcerer => [Ability::Charisma, Ability::Constitution],
            Class::Warlock => [Ability::Charisma, Ability::Wisdom],
            Class::Wizard => [Ability::Intelligence, Ability::Wisdom]
        }
    }

    pub fn saves_on(&self, ability: Ability) -> bool {
        let st = self.saving_throws();
        st[0] == ability || st[1] == ability
    }

    pub fn skills_choice(&self) -> Choice<Skill> {
        match *self {
            Class::Barbarian => Choice::new(2, vec![
                Skill::AnimalHandling, Skill::Athletics,
                Skill::Intimidation, Skill::Nature,
                Skill::Perception, Skill::Survival]),
            Class::Bard => Choice::new(3, Skill::all()),
            Class::Cleric => Choice::new(2, vec![
                Skill::History, Skill::Insight, Skill::Medicine,
                Skill::Persuasion, Skill::Religion]),
            Class::Druid => Choice::new(2, vec![
                Skill::Arcana, Skill::AnimalHandling, Skill::Insight,
                Skill::Medicine, Skill::Nature, Skill::Perception,
                Skill::Religion, Skill::Survival]),
            Class::Fighter =>Choice::new(2, vec![
                Skill::Acrobatics, Skill::AnimalHandling,
                Skill::Athletics, Skill::History, Skill::Insight,
                Skill::Intimidation, Skill::Perception, Skill::Survival]),
            Class::Monk => Choice::new(2, vec![
                Skill::Acrobatics, Skill::Athletics, Skill::History,
                Skill::Insight, Skill::Religion, Skill::Stealth]),
            Class::Paladin => Choice::new(2, vec![
                Skill::Athletics, Skill::Insight, Skill::Intimidation,
                Skill::Medicine, Skill::Persuasion, Skill::Religion]),
            Class::Ranger => Choice::new(3, vec![
                Skill::AnimalHandling, Skill::Athletics, Skill::Insight,
                Skill::Investigation, Skill::Nature, Skill::Perception,
                Skill::Stealth, Skill::Survival]),
            Class::Rogue => Choice::new(4, vec![
                Skill::Acrobatics, Skill::Athletics, Skill::Deception,
                Skill::Insight, Skill::Intimidation, Skill::Investigation,
                Skill::Perception, Skill::Performance, Skill::Persuasion,
                Skill::SleightOfHand, Skill::Stealth]),
            Class::Sorcerer => Choice::new(2, vec![
                Skill::Arcana, Skill::Deception, Skill::Insight,
                Skill::Intimidation, Skill::Persuasion, Skill::Religion]),
            Class::Warlock => Choice::new(2, vec![
                Skill::Arcana, Skill::Deception, Skill::History,
                Skill::Intimidation, Skill::Investigation,
                Skill::Nature, Skill::Religion]),
            Class::Wizard => Choice::new(2, vec![
                Skill::Arcana, Skill::History, Skill::Insight,
                Skill::Investigation, Skill::Medicine, Skill::Religion])
        }
    }

    // features

    pub fn proficiency_points(&self, level: usize) -> usize {
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
