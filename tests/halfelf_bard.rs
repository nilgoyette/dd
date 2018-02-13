
extern crate dd;

use dd::{
    Alignment, Background, Character, Class, Language, Race, RaceSize, Skill};
use dd::test::{check_saving_throws, check_skill_modifiers};

#[test]
fn test_halfelf_bard() {
    let c = Character::standard(
        String::from("halfelf_bard"),
        Race::HalfElf, Class::Bard,
        Background::Entertainer, Alignment::NeutralGood,
        vec![Skill::Acrobatics, Skill::Arcana, Skill::Deception,
             Skill::Insight, Skill::Perception, Skill::Performance,
             Skill::Persuasion],
        8, 16, 14, 12, 10, 16);

    assert_eq!(c.size(), RaceSize::Medium);
    assert!(c.can_comprehend(Language::Common));
    assert!(c.can_comprehend(Language::Elvish));
    assert_eq!(c.languages.len(), 3);
    assert_eq!(c.ac(), 13); // Because no armor
    assert_eq!(c.max_hp, 10);
    assert_eq!(c.current_hp, 10);
    assert_eq!(c.temporary_hp, 0);
    assert_eq!(c.initiative(), 3);
    assert_eq!(c.speed(), 30);
    assert_eq!(c.exp, 0);
    check_saving_throws(&c, -1, 5, 2, 1, 0, 5);
    check_skill_modifiers(
        &c, 5, 0, 3, -1, 5, 1, 2, 3, 1, 0, 1, 2, 5, 5, 1, 3, 3, 0);
    assert_eq!(c.darkvision(), 60);
    assert_eq!(c.passive_perception(), 12);
}
