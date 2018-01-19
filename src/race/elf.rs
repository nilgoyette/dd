
use race::RaceFunctions;

pub struct Elf;
impl RaceFunctions for Elf {
    fn darkvision(&self) -> usize {
        60
    }
}

pub struct HighElf;
impl RaceFunctions for HighElf {
    fn darkvision(&self) -> usize {
        60
    }
}

pub struct WoodElf;
impl RaceFunctions for WoodElf {
    fn darkvision(&self) -> usize {
        60
    }
}

pub struct DarkElf;
impl RaceFunctions for DarkElf {
    fn darkvision(&self) -> usize {
        120
    }
}
