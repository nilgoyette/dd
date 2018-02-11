
extern crate dd;

use dd::{
    Alignment, Background, Character, Class, Language, Race, RaceSize, Skill};
use dd::test::{check_saving_throws, check_skill_modifiers};

#[test]
fn test_dragonborn_sorcerer() {
    let languages = Language::from(
        Race::Dragonborn, Class::Sorcerer, Background::Outlander
    ).auto_select();
    let c = Character::new(
        String::from("sorcerer"),
        Race::Dragonborn, Class::Sorcerer,
        Background::Outlander, Alignment::NeutralGood, languages,
        vec![Skill::Arcana, Skill::Athletics,
             Skill::Intimidation, Skill::Survival],
        10, 13, 14, 10, 12, 16);

    assert_eq!(c.size(), RaceSize::Medium);
    assert!(c.can_comprehend(Language::Common));
    assert!(c.can_comprehend(Language::Draconic));
    assert_eq!(c.languages.len(), 3);
    assert_eq!(c.ac(), 14);
    // TODO Draconic Resilience +1 HP
    assert_eq!(c.max_hp, 8);
    assert_eq!(c.current_hp, 8);
    assert_eq!(c.temporary_hp, 0);
    assert_eq!(c.initiative(), 1);
    assert_eq!(c.speed(), 30);
    assert_eq!(c.exp, 0);
    check_saving_throws(&c, 0, 1, 4, 0, 1, 5);
    check_skill_modifiers(
        &c, 1, 1, 2, 2, 3, 0, 1, 5, 0, 1, 0, 1, 3, 3, 0, 1, 1, 3);
    assert_eq!(c.darkvision(), 0);
    assert_eq!(c.passive_perception(), 11);
}
