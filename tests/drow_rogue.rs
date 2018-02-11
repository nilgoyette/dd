
extern crate dd;

use dd::{
    Alignment, Background, Character, Class, Language, Race, RaceSize, Skill};
use dd::test::{check_saving_throws, check_skill_modifiers};

#[test]
fn test_drow_rogue() {
    let c = Character::standard(
        String::from("drow_rogue"),
        Race::DarkElf, Class::Rogue,
        Background::Charlatan, Alignment::Neutral,
        vec![Skill::Deception, Skill::Insight, Skill::Investigation,
             Skill::Perception, Skill::Persuasion, Skill::SleightOfHand,
             Skill::Stealth],
        8, 16, 10, 13, 12, 16);

    assert_eq!(c.size(), RaceSize::Medium);
    assert!(c.can_comprehend(Language::Common));
    assert!(c.can_comprehend(Language::Elvish));
    assert!(c.can_comprehend(Language::Undercommon));
    assert!(c.can_comprehend(Language::ThievesCant));
    assert_eq!(c.languages.len(), 4);
    assert_eq!(c.ac(), 13);
    assert_eq!(c.max_hp, 8);
    assert_eq!(c.current_hp, 8);
    assert_eq!(c.temporary_hp, 0);
    assert_eq!(c.initiative(), 3);
    assert_eq!(c.speed(), 30);
    assert_eq!(c.exp, 0);
    check_saving_throws(&c, -1, 5, 0, 3, 1, 3);
    check_skill_modifiers(
        &c, 3, 1, 1, -1, 5, 1, 3, 3, 3, 1, 1, 3, 3, 5, 1, 5, 5, 1);
    assert_eq!(c.darkvision(), 120);
    // TODO Should be 15, because "Expertise" rogue feature.
    // 10(base) + 1(dex) + 2(expertise) * 2(proficiency bonus)
    assert_eq!(c.passive_perception(), 13);
}
