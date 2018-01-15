
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Bard;

impl ClassFunctions for Bard {
    fn hit_dice(&self) -> HitDice {
        HitDice::D8
    }

    fn armors(&self) -> Vec<ArmorProficiency> {
        vec![ArmorProficiency::Light]
    }

    fn weapons(&self) -> Vec<WeaponProficiency> {
        let mut weapons = Vec::with_capacity(19);
        weapons.extend(WeaponProficiency::simple());
        weapons.extend(vec![
            WeaponProficiency::HandCrossbow,
            WeaponProficiency::LongSword,
            WeaponProficiency::Rapier,
            WeaponProficiency::ShortSword]);
        weapons
    }

    fn saving_throws(&self) -> [Ability; 2] {
        [Ability::Charisma, Ability::Dexterity]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(vec![], 3, Skill::all())
    }
}
