
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Cleric;

impl ClassFunctions for Cleric {
    fn hit_dice(&self) -> HitDice {
        HitDice::D8
    }

    fn armors(&self) -> Vec<ArmorProficiency> {
        vec![ArmorProficiency::Light,
             ArmorProficiency::Medium,
             ArmorProficiency::Shield]
    }

    fn weapons(&self) -> Vec<WeaponProficiency> {
        WeaponProficiency::simple()
    }

    fn saving_throws(&self) -> [Ability; 2] {
        [Ability::Charisma, Ability::Wisdom]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(2, vec![], vec![
			Skill::History, Skill::Insight, Skill::Medicine,
			Skill::Persuasion, Skill::Religion])
    }
}
