
use race::{RaceFunctions, RaceSize};

pub struct Elf;
impl RaceFunctions for Elf {
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

pub struct HalfElf;
impl RaceFunctions for HalfElf {
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
