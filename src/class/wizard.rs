
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Wizard;

impl ClassFunctions for Wizard {
    fn hit_dice(&self) -> HitDice {
        HitDice::D6
    }

    fn armors(&self) -> Vec<ArmorProficiency> {
        vec![]
    }

    fn weapons(&self) -> Vec<WeaponProficiency> {
        vec![WeaponProficiency::Dagger, WeaponProficiency::Dart,
             WeaponProficiency::Sling, WeaponProficiency::QuarterStaff,
             WeaponProficiency::LightCrossbow]
    }

    fn saving_throws(&self) -> [Ability; 2] {
        [Ability::Intelligence, Ability::Wisdom]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(2, vec![], vec![
			Skill::Arcana, Skill::History, Skill::Insight,
			Skill::Investigation, Skill::Medicine, Skill::Religion])
    }
}
