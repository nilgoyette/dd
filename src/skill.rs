
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
        match background {
            Background::Acolyte =>
                Selections::forced(vec![Skill::Insight, Skill::Religion]),
            Background::Anthropologist =>
                Selections::forced(vec![Skill::Insight, Skill::Religion]),
            Background::Archaeologist =>
                Selections::forced(vec![Skill::History, Skill::Survival]),
            Background::BlackFistDoubleAgent =>
                Selections::forced(vec![Skill::Deception, Skill::Insight]),
            Background::CaravanSpecialist =>
                Selections::forced(vec![Skill::AnimalHandling,
                                        Skill::Survival]),
            Background::Charlatan =>
                Selections::forced(vec![Skill::Deception,
                                        Skill::SleightOfHand]),
            Background::CityWatch =>
                Selections::forced(vec![Skill::Athletics, Skill::Insight]),
            Background::ClanCrafter =>
                Selections::forced(vec![Skill::History, Skill::Insight]),
            Background::CloisteredScholar =>
                Selections::new(
                    vec![Skill::History], 1, vec![Skill::Arcana,
                                                  Skill::Nature,
                                                  Skill::Religion]),
            Background::CormanthorRefugee =>
                Selections::forced(vec![Skill::Nature, Skill::Survival]),
            Background::Courtier =>
                Selections::forced(vec![Skill::Insight, Skill::Persuasion]),
            Background::Criminal =>
                Selections::forced(vec![Skill::Deception, Skill::Stealth]),
            Background::Dissenter =>
                Selections::forced(vec![]),
            Background::DragonCasualty =>
                Selections::forced(vec![Skill::Intimidation, Skill::Survival]),
            Background::EarthspurMiner =>
                Selections::forced(vec![Skill::Athletics, Skill::Survival]),
            Background::Entertainer =>
                Selections::forced(vec![Skill::Acrobatics,
                                        Skill::Performance]),
            // TODO Faction Agent's second skill depends on the
            // character's faction. "one Intelligence, Wisdom, or Charisma
            // skill of your choice, as appropriate to your faction "
            Background::FactionAgent => {
                let mut choices = Ability::Charisma.skills();
                choices.extend(Ability::Intelligence.skills());
                choices.extend(Ability::Wisdom.skills());
                Selections::new(vec![Skill::Insight], 1, choices)
            },
            Background::FarTraveler =>
                Selections::forced(vec![Skill::Insight, Skill::Perception]),
            Background::FolkHero =>
                Selections::forced(vec![Skill::AnimalHandling,
                                        Skill::Survival]),
            Background::GateUrchin =>
                Selections::forced(vec![Skill::Deception,
                                        Skill::SleightOfHand]),
            Background::Gladiator =>
                Selections::forced(vec![Skill::Acrobatics,
                                        Skill::Performance]),
            Background::GuildArtisan =>
                Selections::forced(vec![Skill::Insight, Skill::Persuasion]),
            Background::GuildMerchant =>
                Selections::forced(vec![Skill::Insight, Skill::Persuasion]),
            Background::Harborfolk =>
                Selections::forced(vec![Skill::Athletics,
                                        Skill::SleightOfHand]),
            Background::HauntedOne =>
                Selections::new(vec![], 2, vec![Skill::Arcana,
                                                Skill::Investigation,
                                                Skill::Religion,
                                                Skill::Survival]),
            Background::Heretic =>
                Selections::forced(vec![Skill::Deception, Skill::Religion]),
            Background::Hermit =>
                Selections::forced(vec![Skill::Medicine, Skill::Religion]),
            Background::HillsfarMerchant =>
                Selections::forced(vec![Skill::Insight, Skill::Persuasion]),
            Background::HillsfarSmuggler =>
                Selections::forced(vec![Skill::Perception, Skill::Stealth]),
            Background::Inheritor =>
                Selections::new(
                    vec![Skill::Survival], 1, vec![Skill::Arcana,
                                                   Skill::History,
                                                   Skill::Religion]),
            Background::Initiate =>
                Selections::forced(vec![Skill::Athletics,
                                        Skill::Intimidation]),
            Background::Inquisitor =>
                Selections::forced(vec![Skill::Investigation,
                                        Skill::Religion]),
            Background::Investigator =>
                Selections::forced(vec![Skill::Insight, Skill::Investigation]),
            Background::IronRouteBandit =>
                Selections::forced(vec![Skill::AnimalHandling,
                                        Skill::Stealth]),
            Background::Knight =>
                Selections::forced(vec![Skill::History, Skill::Persuasion]),
            Background::KnightOfTheOrder =>
                Selections::new(
                    vec![Skill::Persuasion], 1, vec![Skill::Arcana,
                                                     Skill::History,
                                                     Skill::Nature,
                                                     Skill::Religion]),
            Background::MercenaryVeteran =>
                Selections::forced(vec![Skill::Athletics, Skill::Persuasion]),
            Background::MulmasterAristocrat =>
                Selections::forced(vec![Skill::Deception, Skill::Performance]),
            Background::Noble =>
                Selections::forced(vec![Skill::History, Skill::Persuasion]),
            Background::Outlander =>
                Selections::forced(vec![Skill::Athletics, Skill::Survival]),
            Background::PhlanInsurgent =>
                Selections::forced(vec![Skill::Stealth, Skill::Survival]),
            Background::PhlanRefugee =>
                Selections::forced(vec![Skill::Athletics, Skill::Insight]),
            Background::Sailor =>
                Selections::forced(vec![Skill::Athletics, Skill::Perception]),
            Background::Sage =>
                Selections::forced(vec![Skill::Arcana, Skill::History]),
            Background::SecretIdentity =>
                Selections::forced(vec![Skill::Deception, Skill::Stealth]),
            Background::ShadeFanatic =>
                Selections::forced(vec![Skill::Deception,
                                        Skill::Intimidation]),
            Background::Soldier =>
                Selections::forced(vec![Skill::Athletics,
                                        Skill::Intimidation]),
            Background::Spy =>
                Selections::forced(vec![Skill::Deception, Skill::Stealth]),
            Background::StojanowPrisoner =>
                Selections::forced(vec![Skill::Deception, Skill::Perception]),
            Background::TicklebellyNomad =>
                Selections::forced(vec![Skill::AnimalHandling, Skill::Nature]),
            Background::TradeSheriff =>
                Selections::forced(vec![Skill::Investigation,
                                        Skill::Persuasion]),
            Background::UrbanBountyHunter =>
                Selections::new(vec![], 2, vec![Skill::Deception,
                                                Skill::Insight,
                                                Skill::Persuasion,
                                                Skill::Stealth]),
            Background::Urchin =>
                Selections::forced(vec![Skill::SleightOfHand, Skill::Stealth]),
            Background::UthgardtTribeMember =>
                Selections::forced(vec![Skill::Athletics, Skill::Survival]),
            Background::Vizier =>
                Selections::forced(vec![Skill::History, Skill::Religion]),
            Background::WaterdhavianNoble =>
                Selections::forced(vec![Skill::History, Skill::Persuasion])
        }
    }
}
