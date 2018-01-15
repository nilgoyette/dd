
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Warlock;

impl ClassFunctions for Warlock {
    fn hit_dice(&self) -> HitDice {
        HitDice::D6
    }

    fn armors(&self) -> Vec<ArmorProficiency> {
        vec![]
    }

    fn weapons(&self) -> Vec<WeaponProficiency> {
        WeaponProficiency::simple()
    }

    fn saving_throws(&self) -> [Ability; 2] {
        [Ability::Charisma, Ability::Wisdom]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(vec![], 2, vec![
			Skill::Arcana, Skill::Deception, Skill::History,
			Skill::Intimidation, Skill::Investigation,
			Skill::Nature, Skill::Religion])
    }
}
