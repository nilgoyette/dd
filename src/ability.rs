
use Skill;

#[derive(Clone, Copy, PartialEq)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

impl Ability {
    pub fn skills(&self) -> Vec<Skill> {
        match *self {
            Ability::Strength => vec![Skill::Athletics],
            Ability::Dexterity => vec![Skill::Acrobatics,
                                       Skill::SleightOfHand,
                                       Skill::Stealth],
            Ability::Constitution => vec![],
            Ability::Intelligence => vec![Skill::Arcana,
                                          Skill::History,
                                          Skill::Investigation,
                                          Skill::Nature,
                                          Skill::Religion],
            Ability::Wisdom => vec![Skill::AnimalHandling,
                                    Skill::Insight,
                                    Skill::Medicine,
                                    Skill::Perception,
                                    Skill::Survival],
            Ability::Charisma => vec![Skill::Deception,
                                      Skill::Intimidation,
                                      Skill::Performance,
                                      Skill::Persuasion]
        }
    }
}

pub trait Modifier {
    fn modifier(self) -> isize;
}

impl Modifier for usize {
    fn modifier(self) -> isize {
        let n = self as isize;
        if n < 10 {
            (n - 11) / 2
        } else {
            (n - 10) / 2
        }
    }
}
