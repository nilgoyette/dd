
extern crate dd;

use dd::{
    Alignment, Background, Character, Class, Language, Race, RaceSize, Skill};
use dd::test::{check_saving_throws, check_skill_modifiers};

#[test]
fn test_human_fighter() {
    let c = Character::new(
        String::from("fighter"),
        Race::Human, Class::Fighter,
        Background::Soldier, Alignment::ChaoticGood,
        Language::from(Race::Human, Background::Soldier).auto_select(),
        vec![Skill::Athletics, Skill::History,
             Skill::Intimidation, Skill::Perception],
        16, 9, 15, 13, 11, 14);

    assert_eq!(c.size(), RaceSize::Medium);
    assert!(c.can_comprehend(Language::Common));
    assert_eq!(c.languages.len(), 2);
    assert_eq!(c.ac(), 9); // Because no armor
    assert_eq!(c.initiative(), -1);
    assert_eq!(c.speed(), 30);
    assert_eq!(c.exp, 0);
    check_saving_throws(&c, 5, -1, 4, 1, 0, 2);
    check_skill_modifiers(
        &c, -1, 0, 1, 5, 2, 3, 0, 4, 1, 0, 1, 2, 2, 2, 1, -1, -1, 0);
    assert_eq!(c.darkvision(), 0);
    assert_eq!(c.passive_perception(), 12);
}
