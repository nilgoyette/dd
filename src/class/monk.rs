
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Monk;

impl ClassFunctions for Monk {
    fn hit_dice(&self) -> HitDice {
        HitDice::D8
    }

    fn armors(&self) -> Vec<ArmorProficiency> {
        vec![]
    }

    fn weapons(&self) -> Vec<WeaponProficiency> {
        let mut weapons = Vec::with_capacity(16);
        weapons.extend(WeaponProficiency::simple());
        weapons.push(WeaponProficiency::ShortSword);
        weapons
    }

    fn saving_throws(&self) -> [Ability; 2] {
        [Ability::Dexterity, Ability::Strength]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(vec![], 2, vec![
			Skill::Acrobatics, Skill::Athletics, Skill::History,
			Skill::Insight, Skill::Religion, Skill::Stealth])
    }
}
