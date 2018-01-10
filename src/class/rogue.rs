
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Rogue;

impl ClassFunctions for Rogue {
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
        [Ability::Dexterity, Ability::Intelligence]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(4, vec![], vec![
			Skill::Acrobatics, Skill::Athletics, Skill::Deception,
			Skill::Insight, Skill::Intimidation, Skill::Investigation,
			Skill::Perception, Skill::Performance, Skill::Persuasion,
			Skill::SleightOfHand, Skill::Stealth])
    }
}
