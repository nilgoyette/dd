
use ability::{Ability, Modifier};
use background::Background;
use class::Class;
use skill::Skill;
use race::Race;
use random::d20;
use {Alignment};

pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub background: Background,
    pub alignment: Alignment,
    pub skills: Vec<Skill>,

    pub strength: usize,
    pub dexterity: usize,
    pub constitution: usize,
    pub intelligence: usize,
    pub wisdom: usize,
    pub charisma: usize,

    // Calculated
    pub level: usize,
    pub proficiency_bonus: usize,
    pub base_ac: isize,

    // Changed often
    pub exp: usize,
    pub max_hp: usize,
    pub temporary_hp: usize,
    pub current_hp: usize
}

impl Character {
    // Gender, photo
    // Race
    // Class et kit selon la race
    // Alignment selon la race et la classe/kit
    // Abilities. Respecter min/max selon race/classe. Différentes méthodes d'allocation des points
    // Skills

    pub fn new(
        name: String,
        race: Race,
        class: Class,
        background: Background,
        alignment: Alignment,
        skills: Vec<Skill>,
        strength: usize,
        dexterity: usize,
        constitution: usize,
        intelligence: usize,
        wisdom: usize,
        charisma: usize,
    ) -> Character {
        let proficiency_bonus = class.proficiency_points(1);
        Character {
            name, race, class, background, alignment, skills,
            strength, dexterity, constitution, intelligence, wisdom, charisma,
            level: 1,
            proficiency_bonus,
            base_ac: 10,
            exp: 0,
            max_hp: 0, // TODO
            temporary_hp: 0, // TODO
            current_hp: 0 // TODO
        }
    }

    pub fn ac(&self) -> isize {
        self.base_ac + self.dexterity.modifier()
    }

    pub fn initiative(&self) -> isize {
        self.dexterity.modifier() // Some features may increase initiative
        // like, Bard's "Jack of All Trades"
    }

    pub fn speed(&self) -> usize {
        0 // TODO
    }

    pub fn modifier_for(&self, ability: Ability) -> isize {
        match ability {
            Ability::Strength => self.strength.modifier(),
            Ability::Dexterity => self.dexterity.modifier(),
            Ability::Constitution => self.constitution.modifier(),
            Ability::Intelligence => self.intelligence.modifier(),
            Ability::Wisdom => self.wisdom.modifier(),
            Ability::Charisma => self.charisma.modifier()
        }
    }

    pub fn saving_throw(&self, ability: Ability) -> isize {
        let base = self.modifier_for(ability.clone());
        let bonus = if self.class.saves_on(ability) {
            self.proficiency_bonus as isize
        } else {
            0
        };
        base + bonus
    }

    pub fn skill_check(&self, skill: Skill) -> isize {
        let base = self.modifier_for(skill.ability());
        let bonus = if self.skills.contains(&skill) {
            self.proficiency_bonus as isize
        } else {
            0
        };
        base + bonus
    }

    pub fn passive_perception(&self) -> isize {
        // TODO other bonuses (Feats like "Observant" (+5 to passive perception)]
        let base = 10;
        base + self.skill_check(Skill::Perception)
    }

    pub fn active_perception(&self) -> isize {
        d20() as isize + self.skill_check(Skill::Perception)
    }
}
