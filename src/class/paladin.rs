
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Paladin;

impl ClassFunctions for Paladin {
    fn hit_dice(&self) -> HitDice {
        HitDice::D10
    }

    fn armors(&self) -> Vec<ArmorProficiency> {
        ArmorProficiency::all()
    }

    fn weapons(&self) -> Vec<WeaponProficiency> {
        WeaponProficiency::all()
    }

    fn saving_throws(&self) -> [Ability; 2] {
        [Ability::Charisma, Ability::Wisdom]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(vec![], 2, vec![
			Skill::Athletics, Skill::Insight, Skill::Intimidation,
			Skill::Medicine, Skill::Persuasion, Skill::Religion])
    }
}
