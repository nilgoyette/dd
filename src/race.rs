
// http://engl393-dnd5th.wikia.com/wiki/D%26D_5E_Adventure_League_Playable_Races
#[derive(Clone, PartialEq)]
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
