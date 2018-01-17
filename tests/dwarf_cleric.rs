
extern crate dd;

use dd::{Alignment, Background, Character, Class, Language, Race, Skill};
use dd::test::{check_saving_throws, check_skill_modifiers};

#[test]
fn test_dwarf_cleric() {
    let c = Character::new(
        String::from("dwarf_cleric"),
        Race::HillDwarf, Class::Cleric,
        Background::GuildArtisan, Alignment::LawfulGood,
        Language::from(Race::HillDwarf, Background::GuildArtisan).auto_select(),
        vec![Skill::Insight, Skill::Medicine,
             Skill::Persuasion, Skill::Religion],
        14, 8, 15, 10, 16, 12);

    assert!(c.can_comprehend(Language::Common));
    assert!(c.can_comprehend(Language::Dwarvish));
    assert_eq!(c.languages.len(), 3);
    assert_eq!(c.ac(), 9);
    assert_eq!(c.initiative(), -1);
    assert_eq!(c.speed(), 30);
    assert_eq!(c.exp, 0);
    check_saving_throws(&c, 2, -1, 2, 0, 5, 3);
    check_skill_modifiers(
        &c, -1, 3, 0, 2, 1, 0, 5, 1, 0, 5, 0, 3, 1, 3, 2, -1, -1, 3);
}
