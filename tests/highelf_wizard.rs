
extern crate dd;

use dd::{Alignment, Background, Character, Class, Language, Race, Skill};
use dd::test::{check_saving_throws, check_skill_modifiers};

#[test]
fn test_highelf_wizard() {
    let c = Character::new(
        String::from("highelf_wizard"),
        Race::HighElf, Class::Wizard,
        Background::Noble, Alignment::LawfulGood,
        Language::from(Race::HighElf, Background::Noble).auto_select(),
        vec![Skill::Arcana, Skill::History, Skill::Investigation,
             Skill::Perception, Skill::Persuasion],
        10, 16, 12, 16, 13, 8);

    assert!(c.can_comprehend(Language::Common));
    assert!(c.can_comprehend(Language::Elvish));
    assert_eq!(c.languages.len(), 3);
    assert_eq!(c.ac(), 13); // Because no armor
    assert_eq!(c.initiative(), 3);
    assert_eq!(c.speed(), 30);
    assert_eq!(c.exp, 0);
    check_saving_throws(&c, 0, 3, 1, 5, 3, -1);
    check_skill_modifiers(
        &c, 3, 1, 5, 0, -1, 5, 1, -1, 5, 1, 3, 3, -1, 1, 3, 3, 3, 1);
        //  Ac An Ar At De Hi In In In Me Na Pe Pe Pe Re Sl St Su
    assert_eq!(c.darkvision(), 60);
}
