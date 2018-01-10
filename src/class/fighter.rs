
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Fighter;

impl ClassFunctions for Fighter {
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
        [Ability::Constitution, Ability::Strength]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(2, vec![], vec![
			Skill::Acrobatics, Skill::AnimalHandling,
			Skill::Athletics, Skill::History, Skill::Insight,
			Skill::Intimidation, Skill::Perception, Skill::Survival])
    }
}
