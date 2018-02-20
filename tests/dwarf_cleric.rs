
extern crate dd;

use dd::{
    Alignment, Background, Character, Class, Language, Race, RaceSize, Skill};
use dd::test::{add_armor, check_saving_throws, check_skill_modifiers};

#[test]
fn test_dwarf_cleric() {
    let mut c = Character::standard(
        String::from("dwarf_cleric"),
        Race::HillDwarf, Class::Cleric,
        Background::GuildArtisan, Alignment::LawfulGood,
        vec![Skill::Insight, Skill::Medicine,
             Skill::Persuasion, Skill::Religion],
        14, 8, 15, 10, 16, 12);
    add_armor(&mut c, "Chain Mail");
    c.offhand = true;

    assert_eq!(c.size(), RaceSize::Medium);
    assert!(c.can_comprehend(Language::Common));
    assert!(c.can_comprehend(Language::Dwarvish));
    assert_eq!(c.languages.len(), 3);
    assert_eq!(c.ac(), 18);
    // TODO Dwarven Toughness, +1 HP per level
    assert_eq!(c.max_hp, 10);
    assert_eq!(c.current_hp, 10);
    assert_eq!(c.temporary_hp, 0);
    assert_eq!(c.initiative(), -1);
    assert_eq!(c.speed(), 25);
    assert_eq!(c.exp, 0);
    check_saving_throws(&c, 2, -1, 2, 0, 5, 3);
    check_skill_modifiers(
        &c, -1, 3, 0, 2, 1, 0, 5, 1, 0, 5, 0, 3, 1, 3, 2, -1, -1, 3);
    assert_eq!(c.darkvision(), 60);
    assert_eq!(c.passive_perception(), 13);
}
