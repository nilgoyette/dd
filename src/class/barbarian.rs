
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Barbarian;

impl ClassFunctions for Barbarian {
    fn hit_dice(&self) -> HitDice {
        HitDice::D12
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
        [Ability::Constitution, Ability::Strength]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(2, vec![], vec![
			Skill::AnimalHandling, Skill::Athletics,
			Skill::Intimidation, Skill::Nature,
			Skill::Perception, Skill::Survival])
    }
}
