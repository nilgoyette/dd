
use {Background, Class, Race, Selections};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Language {
    /// Demons, chaotic evil outsiders [Exotic]
    Abyssal,
    /// Water-based
    Aquan,
    /// Air-based
    Auran,
    /// Celestials (angels, devas) [Exotic]
    Celestial,
    /// Humans, halflings, half-elves, half-orcs [Standard]
    Common,
    /// Mind flayers, beholders [Exotic]
    DeepSpeech,
    /// Kobolds, troglodytes, lizardfolk, dragons, dragonborn [Exotic]
    Draconic,
    /// Druids (only) [Exotic]
    Druidic,
    /// Dwarves [Standard]
    Dwarvish,
    /// Elves [Standard]
    Elvish,
    /// Ogres, giants [Standard]
    Giant,
    /// Gnomes [Standard]
    Gnomish,
    /// Goblinoids, hobgoblins, bugbears [Standard]
    Goblin,
    /// Halflings [Standard]
    Halfling,
    /// Fire-based
    Ignan,
    /// Devils, Tieflings [Exotic]
    Infernal,
    /// Netherese
    Netherese,
    /// Orcs [Standard]
    Orc,
    /// Elementals [Exotic]
    Primordial,
    /// Fey creatures (dryads, brownies, leprechauns) [Exotic]
    Sylvan,
    /// Earth-based
    Terran,
    /// Drow, Underdark traders [Exotic]
    Undercommon,
    // From specific skills. Do NOT add in Language::all() because they must
    // never appear in a Selection::choice.
    ThievesCant
}

// http://engl393-dnd5th.wikia.com/wiki/Backgrounds
impl Language {
    pub fn all() -> Vec<Language> {
        vec![Language::Abyssal, Language::Aquan, Language::Auran,
             Language::Celestial, Language::Common, Language::DeepSpeech,
             Language::Draconic, Language::Druidic, Language::Dwarvish,
             Language::Elvish, Language::Giant, Language::Gnomish,
             Language::Goblin, Language::Halfling, Language::Ignan,
             Language::Infernal, Language::Netherese, Language::Orc,
             Language::Primordial, Language::Sylvan, Language::Terran,
             Language::Undercommon
        ]
    }

    /// Returns `true` if the speaker of a language can effectively communicate
    /// with the speaker of another language.
    /// In short: true if same language OR dialect of the same language.
    pub fn can_comprehend(&self, rhs: Language) -> bool {
        if *self == rhs {
            return true;
        }

        let is_primordial = |l: Language| {
               l == Language::Primordial
            || l == Language::Aquan
            || l == Language::Auran
            || l == Language::Ignan
            || l == Language::Terran
        };
        is_primordial(*self) == is_primordial(rhs)
    }

    pub fn from(
        race: Race,
        class: Class,
        background: Background
    ) -> Selections<Language> {
        let r = Language::from_race(race);
        let c = Language::from_class(class);
        let b = Language::bonus_from_background(background);
        r + c + b
    }

    /// Returns the selection of language(s) (standard and choosen) that a
    /// specific race is supposed to comprehend.
    pub fn from_race(race: Race) -> Selections<Language> {
        let mut s = Selections::new(
            vec![Language::Common], 0, Language::all());
        match race {
            Race::Aasimar => s.add_forced(Language::Celestial),
            Race::Bugbear => s.add_forced(Language::Goblin),
            Race::Dragonborn => s.add_forced(Language::Draconic),
            Race::Dwarf | Race::HillDwarf | Race::MountainDwarf
                => s.add_forced(Language::Dwarvish),
            Race::DuergarDwarf => {
                s.add_forced(Language::Dwarvish);
                s.add_forced(Language::Undercommon);
            },
            Race::Elf | Race:: HighElf | Race::WoodElf
                => s.add_forced(Language::Elvish),
            Race::DarkElf => {
                s.add_forced(Language::Elvish);
                s.add_forced(Language::Undercommon);
            },
            Race::Firbolg => {
                s.add_forced(Language::Elvish);
                s.add_forced(Language::Giant);
            },
            Race::Genasi => {},
            Race::AirGenasi => s.add_forced(Language::Auran),
            Race::EarthGenasi => s.add_forced(Language::Terran),
            Race::FireGenasi => s.add_forced(Language::Ignan),
            Race::WaterGenasi => s.add_forced(Language::Aquan),
            Race::Gnome | Race::ForestGnome | Race::RockGnome
                => s.add_forced(Language::Gnomish),
            Race::DeepGnome => {
                s.add_forced(Language::Gnomish);
                s.add_forced(Language::Undercommon);
            },
            Race::Goblin => s.add_forced(Language::Goblin),
            Race::Goliath => s.add_forced(Language::Giant),
            Race::HalfElf => s.add_forced(Language::Elvish),
            Race::HalfOrc => s.add_forced(Language::Orc),
            Race::Halfling | Race::GhostwiseHalfling |
            Race::LightfootHalfling | Race::StoutHalfling
                => s.add_forced(Language::Halfling),
            Race::Hobgoblin => s.add_forced(Language::Goblin),
            Race::Human => {},
            Race::Kenku => s.add_forced(Language::Auran),
            Race::Kobold => s.add_forced(Language::Draconic),
            Race::Lizardfolk => s.add_forced(Language::Draconic),
            Race::Orc => s.add_forced(Language::Orc),
            Race::Tabaxi => {},
            Race::Tiefling => s.add_forced(Language::Infernal),
            Race::Tortle => s.add_forced(Language::Aquan),
            Race::Triton => s.add_forced(Language::Primordial),
            Race::YuanTiPureblood => {
                s.add_forced(Language::Abyssal);
                s.add_forced(Language::Draconic);
            }
        }

        // Some races have 1 random language choice
        if race == Race::HalfElf ||
           race == Race::Human ||
           race == Race::Tabaxi {
            s.nb_choices += 1;
        }

        s
    }

    pub fn from_class(class: Class) -> Selections<Language> {
        match class {
            Class::Rogue => Selections::forced(vec![Language::ThievesCant]),
            _ => Selections::new(vec![], 0, vec![])
        }
    }

    /// Returns the selection of language(s) (standard and choosen) that a
    /// specific background is supposed to comprehend.
    pub fn bonus_from_background(
        background: Background
    ) -> Selections<Language> {
        match background {
            Background::CormanthorRefugee | Background::TradeSheriff
                => Selections::forced(vec![Language::Elvish]),
            Background::DragonCasualty
                => Selections::forced(vec![Language::Draconic]),
            Background::EarthspurMiner
                => Selections::forced(vec![Language::Dwarvish,
                                           Language::Undercommon]),
            Background::ShadeFanatic
                => Selections::forced(vec![Language::Netherese]),
            Background::TicklebellyNomad
                => Selections::forced(vec![Language::Giant]),
            Background::Acolyte | Background::Anthropologist |
            Background::CityWatch | Background::CloisteredScholar |
            Background::Courtier | Background::FactionAgent |
            Background::Heretic | Background::Investigator | Background::Sage
                => Selections::choices(2, Language::all()),
            Background::Archaeologist | Background::CaravanSpecialist |
            Background::ClanCrafter | Background::FarTraveler |
            Background::GuildArtisan | Background::GuildMerchant |
            Background::HauntedOne | Background::Hermit |
            Background::HillsfarSmuggler | Background::Knight |
            Background::KnightOfTheOrder | Background::Noble |
            Background::Outlander | Background::PhlanRefugee |
            Background::UthgardtTribeMember | Background::WaterdhavianNoble
                => Selections::choices(1, Language::all()),
            // All others only learn their respective racial language
            _   => Selections::new(vec![], 0, vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thieves_cant() {
        let tc = Language::ThievesCant;
        let s = Selections::choices(1, Language::all());
        assert_eq!(s.forced.iter().find(|&&x| x == tc), None);
        assert_eq!(s.choices.iter().find(|&&x| x == tc), None);
    }
}
