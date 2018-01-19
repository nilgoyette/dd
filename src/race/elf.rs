
use race::{RaceFunctions, RaceSize};

pub struct Elf;
impl RaceFunctions for Elf {
    fn base_ac(&self) -> usize {
        10
    }

    fn darkvision(&self) -> usize {
        60
    }

    fn size(&self) -> RaceSize {
        RaceSize::Medium
    }

    fn speed(&self) -> usize {
        30
    }
}

pub struct HighElf;
impl RaceFunctions for HighElf {
    fn base_ac(&self) -> usize {
        10
    }

    fn darkvision(&self) -> usize {
        60
    }

    fn size(&self) -> RaceSize {
        RaceSize::Medium
    }

    fn speed(&self) -> usize {
        30
    }
}

pub struct WoodElf;
impl RaceFunctions for WoodElf {
    fn base_ac(&self) -> usize {
        10
    }

    fn darkvision(&self) -> usize {
        60
    }

    fn size(&self) -> RaceSize {
        RaceSize::Medium
    }

    fn speed(&self) -> usize {
        30
    }
}

pub struct DarkElf;
impl RaceFunctions for DarkElf {
    fn base_ac(&self) -> usize {
        10
    }

    fn darkvision(&self) -> usize {
        120
    }

    fn size(&self) -> RaceSize {
        RaceSize::Medium
    }

    fn speed(&self) -> usize {
        30
    }
}
