
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Ranger;

impl ClassFunctions for Ranger {
    fn hit_dice(&self) -> HitDice {
        HitDice::D10
    }

    fn armors(&self) -> Vec<ArmorProficiency> {
        vec![ArmorProficiency::Light,
             ArmorProficiency::Medium,
             ArmorProficiency::Shield]
    }

    fn weapons(&self) -> Vec<WeaponProficiency> {
        WeaponProficiency::all()
    }

    fn saving_throws(&self) -> [Ability; 2] {
        [Ability::Dexterity, Ability::Strength]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(vec![], 3, vec![
			Skill::AnimalHandling, Skill::Athletics, Skill::Insight,
			Skill::Investigation, Skill::Nature, Skill::Perception,
			Skill::Stealth, Skill::Survival])
    }
}
