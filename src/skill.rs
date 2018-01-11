
use ability::Ability;
use background::Background;

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
    pub fn from_background(
        background: Background
    ) -> (Vec<Skill>, (usize, Vec<Skill>)) {
        let no_choice = (0, vec![]);
        match background {
            Background::Acolyte
                => (vec![Skill::Insight, Skill::Religion], no_choice),
            Background::Anthropologist
                => (vec![Skill::Insight, Skill::Religion], no_choice),
            Background::Archaeologist
                => (vec![Skill::History, Skill::Survival], no_choice),
            Background::BlackFistDoubleAgent
                => (vec![Skill::Deception, Skill::Insight], no_choice),
            Background::CaravanSpecialist
                => (vec![Skill::AnimalHandling, Skill::Survival], no_choice),
            Background::Charlatan
                => (vec![Skill::Deception, Skill::SleightOfHand], no_choice),
            Background::CityWatch
                => (vec![Skill::Athletics, Skill::Insight], no_choice),
            Background::ClanCrafter
                => (vec![Skill::History, Skill::Insight], no_choice),
            Background::CloisteredScholar
                => (vec![Skill::History],  (1, vec![Skill::Arcana,
                                                    Skill::Nature,
                                                    Skill::Religion])),
            Background::CormanthorRefugee
                => (vec![Skill::Nature, Skill::Survival], no_choice),
            Background::Courtier
                => (vec![Skill::Insight, Skill::Persuasion], no_choice),
            Background::Criminal
                => (vec![Skill::Deception, Skill::Stealth], no_choice),
            Background::Dissenter
                => (vec![], no_choice),
            Background::DragonCasualty
                => (vec![Skill::Intimidation, Skill::Survival], no_choice),
            Background::EarthspurMiner
                => (vec![Skill::Athletics, Skill::Survival], no_choice),
            Background::Entertainer
                => (vec![Skill::Acrobatics, Skill::Performance], no_choice),
            // TODO Faction Agent's second skill depends on the
            // character's faction. "one Intelligence, Wisdom, or Charisma
            // skill of your choice, as appropriate to your faction "
            Background::FactionAgent => (vec![Skill::Insight], no_choice),
            Background::FarTraveler
                => (vec![Skill::Insight, Skill::Perception], no_choice),
            Background::FolkHero
                => (vec![Skill::AnimalHandling, Skill::Survival], no_choice),
            Background::GateUrchin
                => (vec![Skill::Deception, Skill::SleightOfHand], no_choice),
            Background::Gladiator
                => (vec![Skill::Acrobatics, Skill::Performance], no_choice),
            Background::GuildArtisan
                => (vec![Skill::Insight, Skill::Persuasion], no_choice),
            Background::GuildMerchant
                => (vec![Skill::Insight, Skill::Persuasion], no_choice),
            Background::Harborfolk
                => (vec![Skill::Athletics, Skill::SleightOfHand], no_choice),
            Background::HauntedOne
                => (vec![], (2, vec![Skill::Arcana,
                                     Skill::Investigation,
                                     Skill::Religion,
                                     Skill::Survival])),
            Background::Heretic
                => (vec![Skill::Deception, Skill::Religion], no_choice),
            Background::Hermit
                => (vec![Skill::Medicine, Skill::Religion], no_choice),
            Background::HillsfarMerchant
                => (vec![Skill::Insight, Skill::Persuasion], no_choice),
            Background::HillsfarSmuggler
                => (vec![Skill::Perception, Skill::Stealth], no_choice),
            Background::Inheritor
                => (vec![Skill::Survival], (1, vec![Skill::Arcana,
                                                    Skill::History,
                                                    Skill::Religion])),
            Background::Initiate
                => (vec![Skill::Athletics, Skill::Intimidation], no_choice),
            Background::Inquisitor
                => (vec![Skill::Investigation, Skill::Religion], no_choice),
            Background::Investigator
                => (vec![Skill::Insight, Skill::Investigation], no_choice),
            Background::IronRouteBandit
                => (vec![Skill::AnimalHandling, Skill::Stealth], no_choice),
            Background::Knight
                => (vec![Skill::History, Skill::Persuasion], no_choice),
            Background::KnightOfTheOrder
                => (vec![Skill::Persuasion], (1, vec![Skill::Arcana,
                                                      Skill::History,
                                                      Skill::Nature,
                                                      Skill::Religion])),
            Background::MercenaryVeteran
                => (vec![Skill::Athletics, Skill::Persuasion], no_choice),
            Background::MulmasterAristocrat
                => (vec![Skill::Deception, Skill::Performance], no_choice),
            Background::Noble
                => (vec![Skill::History, Skill::Persuasion], no_choice),
            Background::Outlander
                => (vec![Skill::Athletics, Skill::Survival], no_choice),
            Background::PhlanInsurgent
                => (vec![Skill::Stealth, Skill::Survival], no_choice),
            Background::PhlanRefugee
                => (vec![Skill::Athletics, Skill::Insight], no_choice),
            Background::Sailor
                => (vec![Skill::Athletics, Skill::Perception], no_choice),
            Background::Sage
                => (vec![Skill::Arcana, Skill::History], no_choice),
            Background::SecretIdentity
                => (vec![Skill::Deception, Skill::Stealth], no_choice),
            Background::ShadeFanatic
                => (vec![Skill::Deception, Skill::Intimidation], no_choice),
            Background::Soldier
                => (vec![Skill::Athletics, Skill::Intimidation], no_choice),
            Background::Spy
                => (vec![Skill::Deception, Skill::Stealth], no_choice),
            Background::StojanowPrisoner
                => (vec![Skill::Deception, Skill::Perception], no_choice),
            Background::TicklebellyNomad
                => (vec![Skill::AnimalHandling, Skill::Nature], no_choice),
            Background::TradeSheriff
                => (vec![Skill::Investigation, Skill::Persuasion], no_choice),
            Background::UrbanBountyHunter
                => (vec![], (2, vec![Skill::Deception,
                                     Skill::Insight,
                                     Skill::Persuasion,
                                     Skill::Stealth])),
            Background::Urchin
                => (vec![Skill::SleightOfHand, Skill::Stealth], no_choice),
            Background::UthgardtTribeMember
                => (vec![Skill::Athletics, Skill::Survival], no_choice),
            Background::Vizier
                => (vec![Skill::History, Skill::Religion], no_choice),
            Background::WaterdhavianNoble
                => (vec![Skill::History, Skill::Persuasion], no_choice)
        }
    }
}
