
use random::d20;
use {
    Ability, Alignment, Armor, Background, Class, ClassFunctions, Language,
    Modifier, Skill, Race, RaceFunctions, RaceSize};

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

    // Equipment
    pub armor: Option<Armor>,
    pub offhand: bool, // Only handle shield for now

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
        charisma: usize
    ) -> Character {
        let race = Race::new(race);
        let class = Class::new(class);
        let hp = class.hit_dice().first_level(constitution);
        Character {
            proficiency_bonus: class.proficiency_points(1),
            name, race, class, background, alignment, languages, skills,
            strength, dexterity, constitution, intelligence, wisdom, charisma,
            level: 1, armor: None, offhand: false, exp: 0,
            max_hp: hp, temporary_hp: 0, current_hp: hp
        }
    }

    pub fn standard(
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
        charisma: usize
    ) -> Character {
        let languages = Language::from(race, class, background).auto_select();
        Character::new(
            name, race, class, background, alignment, languages, skills,
            strength, dexterity, constitution, intelligence, wisdom, charisma)
    }

    pub fn size(&self) -> RaceSize {
        self.race.size()
    }

    pub fn can_comprehend(&self, language: Language) -> bool {
        self.languages.contains(&language)
    }

    pub fn ac(&self) -> isize {
        // https://merricb.com/2014/09/13/armour-class-in-dungeons-dragons-5e/
        let dex_mod = self.dexterity.modifier();
        let shield_modifier = if self.offhand {
            2
        } else {
            0
        };
        let base_ac = match self.armor {
            Some(ref armor) => armor.base_ac(dex_mod),
            None => 10 + dex_mod
        };
        base_ac + shield_modifier
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
