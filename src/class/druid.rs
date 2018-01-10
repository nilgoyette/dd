
use class::ClassFunctions;
use {Ability, ArmorProficiency, HitDice, Selections, Skill, WeaponProficiency};

pub struct Druid;

impl ClassFunctions for Druid {
    fn hit_dice(&self) -> HitDice {
        HitDice::D8
    }

    fn armors(&self) -> Vec<ArmorProficiency> {
        vec![ArmorProficiency::Light,
             ArmorProficiency::Medium,
             ArmorProficiency::Shield]
    }

    fn weapons(&self) -> Vec<WeaponProficiency> {
        vec![WeaponProficiency::Club, WeaponProficiency::Dagger,
             WeaponProficiency::Dart, WeaponProficiency::Javelin,
             WeaponProficiency::Mace, WeaponProficiency::QuarterStaff,
             WeaponProficiency::Scimitar, WeaponProficiency::Sickle,
             WeaponProficiency::Sling, WeaponProficiency::Spear]
    }

    fn saving_throws(&self) -> [Ability; 2] {
        [Ability::Intelligence, Ability::Wisdom]
    }

    fn skills_choice(&self) -> Selections<Skill> {
        Selections::new(2, vec![], vec![
			Skill::Arcana, Skill::AnimalHandling, Skill::Insight,
			Skill::Medicine, Skill::Nature, Skill::Perception,
			Skill::Religion, Skill::Survival])
    }
}
