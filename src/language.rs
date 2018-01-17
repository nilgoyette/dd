
use {Background, Race, Selections};

#[derive(Clone, Copy, PartialEq)]
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
    Undercommon
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
        is_primordial(self.clone()) == is_primordial(rhs)
    }

    pub fn from(race: Race, background: Background) -> Selections<Language> {
        let r = Language::from_race(race);
        let b = Language::bonus_from_background(background);
        r + b
    }

    /// Returns the selection of language(s) (standard and choosen) that a
    /// specific race is supposed to comprehend.
    pub fn from_race(race: Race) -> Selections<Language> {
        fn common_and(mut languages: Vec<Language>) -> Selections<Language> {
            languages.push(Language::Common);
            Selections::forced(languages)
        }
        fn common_and_choice(
            mut languages: Vec<Language>
        ) -> Selections<Language> {
            languages.push(Language::Common);
            Selections::new(languages, 1, Language::all())
        }

        match race {
            Race::Aasimar
                => common_and(vec![Language::Celestial]),
            Race::Bugbear
                => common_and(vec![Language::Goblin]),
            Race::Dragonborn
                => common_and(vec![Language::Draconic]),
            Race::Dwarf | Race::HillDwarf | Race::MountainDwarf
                => common_and(vec![Language::Dwarvish]),
            Race::DuergarDwarf
                => common_and(vec![Language::Dwarvish, Language::Undercommon]),
            Race::Elf | Race:: HighElf | Race::WoodElf
                => common_and(vec![Language::Elvish]),
            Race::DarkElf
                => Selections::forced(vec![Language::Elvish,
                                           Language::Undercommon]),
            Race::Firbolg
                => common_and(vec![Language::Elvish, Language::Giant]),
            Race::Genasi
                => common_and(vec![]),
            Race::AirGenasi
                => Selections::forced(vec![Language::Auran]),
            Race::EarthGenasi
                => Selections::forced(vec![Language::Terran]),
            Race::FireGenasi
                => Selections::forced(vec![Language::Ignan]),
            Race::WaterGenasi
                => Selections::forced(vec![Language::Aquan]),
            Race::Gnome | Race::ForestGnome | Race::RockGnome
                => common_and(vec![Language::Gnomish]),
            Race::DeepGnome
                => common_and(vec![Language::Gnomish, Language::Undercommon]),
            Race::Goblin
                => common_and(vec![Language::Goblin]),
            Race::Goliath
                => common_and(vec![Language::Giant]),
            Race::HalfElf
                => common_and_choice(vec![Language::Elvish]),
            Race::HalfOrc
                => common_and(vec![Language::Orc]),
            Race::Halfling | Race::GhostwiseHalfling |
            Race::LightfootHalfling | Race::StoutHalfling
                => common_and(vec![Language::Halfling]),
            Race::Hobgoblin
                => common_and(vec![Language::Goblin]),
            Race::Human
                => common_and_choice(vec![]),
            Race::Kenku
                => common_and(vec![Language::Auran]),
            Race::Kobold
                => common_and(vec![Language::Draconic]),
            Race::Lizardfolk
                => common_and(vec![Language::Draconic]),
            Race::Orc
                => common_and(vec![Language::Orc]),
            Race::Tabaxi
                => common_and_choice(vec![]),
            Race::Tiefling
                => common_and(vec![Language::Infernal]),
            Race::Tortle
                => common_and(vec![Language::Aquan]),
            Race::Triton
                => common_and(vec![Language::Primordial]),
            Race::YuanTiPureblood
                => common_and(vec![Language::Abyssal, Language::Draconic]),
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
