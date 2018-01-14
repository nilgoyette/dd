
mod aasimar;
mod darkelf;
mod dragonborn;
mod human;

pub use self::aasimar::Aasimar;
pub use self::darkelf::DarkElf;
pub use self::dragonborn::Dragonborn;
pub use self::human::Human;

// http://engl393-dnd5th.wikia.com/wiki/D%26D_5E_Adventure_League_Playable_Races
#[derive(Clone, Copy, PartialEq)]
pub enum Race {
    Aasimar,
    Bugbear,
    Dragonborn,
    Dwarf, DuergarDwarf, HillDwarf, MountainDwarf,
    Elf, HighElf, WoodElf, DarkElf,
    Firbolg, // MUST be a member of The Harpers or The Emerald Enclave 
    Genasi, AirGenasi, EarthGenasi, FireGenasi, WaterGenasi,
    Gnome, ForestGnome, RockGnome, DeepGnome,
    Goblin,
    Goliath,
    HalfElf,
    HalfOrc,
    Halfling, GhostwiseHalfling, LightfootHalfling, StoutHalfling,
    Hobgoblin,
    Human,
    Kenku,
    Kobold,
    Lizardfolk,
    Orc,
    Tabaxi,
    Tiefling,
    Tortle,
    Triton,
    YuanTiPureblood
}

impl Race {
    pub fn new(self) -> Box<RaceFunctions> {
        match self {
            Race::Aasimar => Box::new(Aasimar {}),
            //Race::Bugbear => Box::new(Bugbear {}),
            Race::Dragonborn => Box::new(Dragonborn {}),
            //Race::Dwarf => Box::new(Dwarf {}),
            //Race::DuergarDwarf => Box::new(DuergarDwarf {}),
            //Race::HillDwarf => Box::new(HillDwarf {}),
            //Race::MountainDwarf => Box::new(MountainDwarf {}),
            //Race::Elf => Box::new(Elf {}),
            //Race::HighElf => Box::new(HighElf {}),
            //Race::WoodElf => Box::new(WoodElf {}),
            Race::DarkElf => Box::new(DarkElf {}),
            //Race::Firbolg => Box::new(Firbolg {}),
            //Race::Genasi => Box::new(Genasi {}),
            //Race::AirGenasi => Box::new(AirGenasi {}),
            //Race::EarthGenasi => Box::new(EarthGenasi {}),
            //Race::FireGenasi => Box::new(FireGenasi {}),
            //Race::WaterGenasi => Box::new(WaterGenasi {}),
            //Race::Gnome => Box::new(Gnome {}),
            //Race::ForestGnome => Box::new(ForestGnome {}),
            //Race::RockGnome => Box::new(RockGnome {}),
            //Race::DeepGnome => Box::new(DeepGnome {}),
            //Race::Goblin => Box::new(Goblin {}),
            //Race::Goliath => Box::new(Goliath {}),
            //Race::HalfElf => Box::new(HalfElf {}),
            //Race::HalfOrc => Box::new(HalfOrc {}),
            //Race::Halfling => Box::new(Halfling {}),
            //Race::GhostwiseHalfling => Box::new(GhostwiseHalfling {}),
            //Race::LightfootHalfling => Box::new(LightfootHalfling {}),
            //Race::StoutHalfling => Box::new(StoutHalfling {}),
            //Race::Hobgoblin => Box::new(Hobgoblin {}),
            Race::Human => Box::new(Human {}),
            //Race::Kenku => Box::new(Kenku {}),
            //Race::Kobold => Box::new(Kobold {}),
            //Race::Lizardfolk => Box::new(Lizardfolk {}),
            //Race::Orc => Box::new(Orc {}),
            //Race::Tabaxi => Box::new(Tabaxi {}),
            //Race::Tiefling => Box::new(Tiefling {}),
            //Race::Tortle => Box::new(Tortle {}),
            //Race::Triton => Box::new(Triton {}),
            //Race::YuanTiPureblood => Box::new(YuanTiPureblood {})
            _ => panic!("Not implemented yet")
        }
    }
}

pub trait RaceFunctions {
    fn base_ac(&self) -> usize {
        10
    }

    fn speed(&self) -> usize {
        30
    }
}