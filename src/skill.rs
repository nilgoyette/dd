
use {Ability, Background, Selections};

#[derive(Clone, Copy, PartialEq)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival
}

impl Skill {
    pub fn all() -> Vec<Skill> {
        vec![Skill::Acrobatics, Skill::AnimalHandling, Skill::Arcana,
             Skill::Athletics, Skill::Deception, Skill::History,
             Skill::Insight, Skill::Intimidation, Skill::Investigation,
             Skill::Medicine, Skill::Nature, Skill::Perception,
             Skill::Performance, Skill::Persuasion, Skill::Religion,
             Skill::SleightOfHand, Skill::Stealth, Skill::Survival]
    }

    // https://www.dandwiki.com/wiki/5e_SRD:Skills
    pub fn ability(&self) -> Ability {
        match *self {
            Skill::Athletics
                => Ability::Strength,
            Skill::Acrobatics |
            Skill::SleightOfHand |
            Skill::Stealth
                => Ability::Dexterity,
            Skill::Arcana |
            Skill::History |
            Skill::Investigation |
            Skill::Nature |
            Skill::Religion
                => Ability::Intelligence,
            Skill::AnimalHandling |
            Skill::Insight |
            Skill::Medicine |
            Skill::Perception |
            Skill::Survival
                => Ability::Wisdom,
            Skill::Deception |
            Skill::Intimidation |
            Skill::Performance |
            Skill::Persuasion
                => Ability::Charisma
        }
    }

    // http://engl393-dnd5th.wikia.com/wiki/Backgrounds
    pub fn from_background(background: Background) -> Selections<Skill> {
        fn only_forced(forced: Vec<Skill>) -> Selections<Skill> {
            Selections::new(0, forced, vec![])
        }

        match background {
            Background::Acolyte
                => only_forced(vec![Skill::Insight, Skill::Religion]),
            Background::Anthropologist
                => only_forced(vec![Skill::Insight, Skill::Religion]),
            Background::Archaeologist
                => only_forced(vec![Skill::History, Skill::Survival]),
            Background::BlackFistDoubleAgent
                => only_forced(vec![Skill::Deception, Skill::Insight]),
            Background::CaravanSpecialist
                => only_forced(vec![Skill::AnimalHandling, Skill::Survival]),
            Background::Charlatan
                => only_forced(vec![Skill::Deception, Skill::SleightOfHand]),
            Background::CityWatch
                => only_forced(vec![Skill::Athletics, Skill::Insight]),
            Background::ClanCrafter
                => only_forced(vec![Skill::History, Skill::Insight]),
            Background::CloisteredScholar
                => Selections::new(
                    1, vec![Skill::History], vec![Skill::Arcana,
                                                  Skill::Nature,
                                                  Skill::Religion]),
            Background::CormanthorRefugee
                => only_forced(vec![Skill::Nature, Skill::Survival]),
            Background::Courtier
                => only_forced(vec![Skill::Insight, Skill::Persuasion]),
            Background::Criminal
                => only_forced(vec![Skill::Deception, Skill::Stealth]),
            Background::Dissenter
                => only_forced(vec![]),
            Background::DragonCasualty
                => only_forced(vec![Skill::Intimidation, Skill::Survival]),
            Background::EarthspurMiner
                => only_forced(vec![Skill::Athletics, Skill::Survival]),
            Background::Entertainer
                => only_forced(vec![Skill::Acrobatics, Skill::Performance]),
            // TODO Faction Agent's second skill depends on the
            // character's faction. "one Intelligence, Wisdom, or Charisma
            // skill of your choice, as appropriate to your faction "
            Background::FactionAgent => {
                let mut choices = Ability::Charisma.skills();
                choices.extend(Ability::Intelligence.skills());
                choices.extend(Ability::Wisdom.skills());
                Selections::new(1, vec![Skill::Insight], choices)
            },
            Background::FarTraveler
                => only_forced(vec![Skill::Insight, Skill::Perception]),
            Background::FolkHero
                => only_forced(vec![Skill::AnimalHandling, Skill::Survival]),
            Background::GateUrchin
                => only_forced(vec![Skill::Deception, Skill::SleightOfHand]),
            Background::Gladiator
                => only_forced(vec![Skill::Acrobatics, Skill::Performance]),
            Background::GuildArtisan
                => only_forced(vec![Skill::Insight, Skill::Persuasion]),
            Background::GuildMerchant
                => only_forced(vec![Skill::Insight, Skill::Persuasion]),
            Background::Harborfolk
                => only_forced(vec![Skill::Athletics, Skill::SleightOfHand]),
            Background::HauntedOne
                => Selections::new(2, vec![], vec![Skill::Arcana,
                                                   Skill::Investigation,
                                                   Skill::Religion,
                                                   Skill::Survival]),
            Background::Heretic
                => only_forced(vec![Skill::Deception, Skill::Religion]),
            Background::Hermit
                => only_forced(vec![Skill::Medicine, Skill::Religion]),
            Background::HillsfarMerchant
                => only_forced(vec![Skill::Insight, Skill::Persuasion]),
            Background::HillsfarSmuggler
                => only_forced(vec![Skill::Perception, Skill::Stealth]),
            Background::Inheritor
                => Selections::new(
                    1, vec![Skill::Survival], vec![Skill::Arcana,
                                                   Skill::History,
                                                   Skill::Religion]),
            Background::Initiate
                => only_forced(vec![Skill::Athletics, Skill::Intimidation]),
            Background::Inquisitor
                => only_forced(vec![Skill::Investigation, Skill::Religion]),
            Background::Investigator
                => only_forced(vec![Skill::Insight, Skill::Investigation]),
            Background::IronRouteBandit
                => only_forced(vec![Skill::AnimalHandling, Skill::Stealth]),
            Background::Knight
                => only_forced(vec![Skill::History, Skill::Persuasion]),
            Background::KnightOfTheOrder
                => Selections::new(
                    1, vec![Skill::Persuasion], vec![Skill::Arcana,
                                                     Skill::History,
                                                     Skill::Nature,
                                                     Skill::Religion]),
            Background::MercenaryVeteran
                => only_forced(vec![Skill::Athletics, Skill::Persuasion]),
            Background::MulmasterAristocrat
                => only_forced(vec![Skill::Deception, Skill::Performance]),
            Background::Noble
                => only_forced(vec![Skill::History, Skill::Persuasion]),
            Background::Outlander
                => only_forced(vec![Skill::Athletics, Skill::Survival]),
            Background::PhlanInsurgent
                => only_forced(vec![Skill::Stealth, Skill::Survival]),
            Background::PhlanRefugee
                => only_forced(vec![Skill::Athletics, Skill::Insight]),
            Background::Sailor
                => only_forced(vec![Skill::Athletics, Skill::Perception]),
            Background::Sage
                => only_forced(vec![Skill::Arcana, Skill::History]),
            Background::SecretIdentity
                => only_forced(vec![Skill::Deception, Skill::Stealth]),
            Background::ShadeFanatic
                => only_forced(vec![Skill::Deception, Skill::Intimidation]),
            Background::Soldier
                => only_forced(vec![Skill::Athletics, Skill::Intimidation]),
            Background::Spy
                => only_forced(vec![Skill::Deception, Skill::Stealth]),
            Background::StojanowPrisoner
                => only_forced(vec![Skill::Deception, Skill::Perception]),
            Background::TicklebellyNomad
                => only_forced(vec![Skill::AnimalHandling, Skill::Nature]),
            Background::TradeSheriff
                => only_forced(vec![Skill::Investigation, Skill::Persuasion]),
            Background::UrbanBountyHunter
                => Selections::new(2, vec![], vec![Skill::Deception,
                                                   Skill::Insight,
                                                   Skill::Persuasion,
                                                   Skill::Stealth]),
            Background::Urchin
                => only_forced(vec![Skill::SleightOfHand, Skill::Stealth]),
            Background::UthgardtTribeMember
                => only_forced(vec![Skill::Athletics, Skill::Survival]),
            Background::Vizier
                => only_forced(vec![Skill::History, Skill::Religion]),
            Background::WaterdhavianNoble
                => only_forced(vec![Skill::History, Skill::Persuasion])
        }
    }
}
