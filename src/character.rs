
use random::d20;
use {Ability, Alignment, Background, Class, ClassFunctions, Language, Modifier,
     Skill, Race, RaceFunctions, RaceSize};

pub struct Character {
    pub name: String,
    pub race: Box<RaceFunctions>,
    pub class: Box<ClassFunctions>,
    pub background: Background,
    pub alignment: Alignment,
    pub languages: Vec<Language>,
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
        languages: Vec<Language>,
        skills: Vec<Skill>,
        strength: usize,
        dexterity: usize,
        constitution: usize,
        intelligence: usize,
        wisdom: usize,
        charisma: usize,
    ) -> Character {
        let race = Race::new(race);
        let class = Class::new(class);
        let hp = class.hit_dice().first_level(constitution);
        Character {
            proficiency_bonus: class.proficiency_points(1),
            base_ac: race.base_ac() as isize,
            name, race, class, background, alignment, languages, skills,
            strength, dexterity, constitution, intelligence, wisdom, charisma,
            level: 1, exp: 0,
            max_hp: hp, temporary_hp: 0, current_hp: hp
        }
    }

    pub fn size(&self) -> RaceSize {
        self.race.size()
    }

    pub fn can_comprehend(&self, language: Language) -> bool {
        self.languages.contains(&language)
    }

    pub fn ac(&self) -> isize {
        self.base_ac + self.dexterity.modifier()
    }

    pub fn initiative(&self) -> isize {
        self.dexterity.modifier() // Some features may increase initiative
        // like, Bard's "Jack of All Trades"
    }

    pub fn speed(&self) -> usize {
        self.race.speed()
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
        let base = self.modifier_for(ability);
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

    pub fn darkvision(&self) -> usize {
        self.race.darkvision()
    }
}
