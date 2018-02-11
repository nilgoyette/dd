
extern crate dd;

use dd::{
    Alignment, Background, Character, Class, Language, Race, RaceSize, Skill};
use dd::test::{check_saving_throws, check_skill_modifiers};

#[test]
fn test_highelf_wizard() {
    let languages = Language::from(
        Race::HighElf, Class::Wizard, Background::Noble
    ).auto_select();
    let c = Character::new(
        String::from("highelf_wizard"),
        Race::HighElf, Class::Wizard,
        Background::Noble, Alignment::LawfulGood, languages,
        vec![Skill::Arcana, Skill::History, Skill::Investigation,
             Skill::Perception, Skill::Persuasion],
        10, 16, 12, 16, 13, 8);

    assert_eq!(c.size(), RaceSize::Medium);
    assert!(c.can_comprehend(Language::Common));
    assert!(c.can_comprehend(Language::Elvish));
    assert_eq!(c.languages.len(), 3);
    assert_eq!(c.ac(), 13); // Because no armor
    assert_eq!(c.max_hp, 7);
    assert_eq!(c.current_hp, 7);
    assert_eq!(c.temporary_hp, 0);
    assert_eq!(c.initiative(), 3);
    assert_eq!(c.speed(), 30);
    assert_eq!(c.exp, 0);
    check_saving_throws(&c, 0, 3, 1, 5, 3, -1);
    check_skill_modifiers(
        &c, 3, 1, 5, 0, -1, 5, 1, -1, 5, 1, 3, 3, -1, 1, 3, 3, 3, 1);
        //  Ac An Ar At De Hi In In In Me Na Pe Pe Pe Re Sl St Su
    assert_eq!(c.darkvision(), 60);
    assert_eq!(c.passive_perception(), 13);
}
