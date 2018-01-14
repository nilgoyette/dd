
extern crate dd;

use dd::{
    Ability, Alignment, Background, Character, Class, Language, Race, Skill};

#[test]
fn test_dragonborn_sorcerer() {
    let c = Character::new(
        String::from("sorcerer"),
        Race::Dragonborn, Class::Sorcerer,
        Background::Outlander, Alignment::NeutralGood,
        Language::from_race(Race::Dragonborn).auto_select(),
        vec![Skill::Arcana, Skill::Athletics,
             Skill::Intimidation, Skill::Survival],
        10, 13, 14, 10, 12, 16);
    assert!(c.can_comprehend(Language::Common));
    assert!(c.can_comprehend(Language::Draconic));
    assert_eq!(c.languages.len(), 2);
    assert_eq!(c.ac(), 14);
    assert_eq!(c.initiative(), 1);
    assert_eq!(c.speed(), 30);
    assert_eq!(c.exp, 0);

    assert_eq!(c.saving_throw(Ability::Strength), 0);
    assert_eq!(c.saving_throw(Ability::Dexterity), 1);
    assert_eq!(c.saving_throw(Ability::Constitution), 4);
    assert_eq!(c.saving_throw(Ability::Intelligence), 0);
    assert_eq!(c.saving_throw(Ability::Wisdom), 1);
    assert_eq!(c.saving_throw(Ability::Charisma), 5);

    assert_eq!(c.skill_check(Skill::Acrobatics), 1);
    assert_eq!(c.skill_check(Skill::AnimalHandling), 1);
    assert_eq!(c.skill_check(Skill::Arcana), 2);
    assert_eq!(c.skill_check(Skill::Athletics), 2);
    assert_eq!(c.skill_check(Skill::Deception), 3);
    assert_eq!(c.skill_check(Skill::History), 0);
    assert_eq!(c.skill_check(Skill::Insight), 1);
    assert_eq!(c.skill_check(Skill::Intimidation), 5);
    assert_eq!(c.skill_check(Skill::Investigation), 0);
    assert_eq!(c.skill_check(Skill::Medicine), 1);
    assert_eq!(c.skill_check(Skill::Nature), 0);
    assert_eq!(c.skill_check(Skill::Perception), 1);
    assert_eq!(c.skill_check(Skill::Performance), 3);
    assert_eq!(c.skill_check(Skill::Persuasion), 3);
    assert_eq!(c.skill_check(Skill::Religion), 0);
    assert_eq!(c.skill_check(Skill::SleightOfHand), 1);
    assert_eq!(c.skill_check(Skill::Stealth), 1);
    assert_eq!(c.skill_check(Skill::Survival), 3);
}
