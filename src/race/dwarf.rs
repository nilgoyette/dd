
use race::{RaceFunctions, RaceSize};

pub struct Dwarf;
impl RaceFunctions for Dwarf {
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
        25
    }
}

pub struct DuergarDwarf;
impl RaceFunctions for DuergarDwarf {
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
        25
    }
}

pub struct HillDwarf;
impl RaceFunctions for HillDwarf {
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
        25
    }
}

pub struct MountainDwarf;
impl RaceFunctions for MountainDwarf {
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
        25
    }
}
