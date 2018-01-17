
use race::RaceFunctions;

pub struct Dwarf;
impl RaceFunctions for Dwarf {
    fn darkvision(&self) -> usize {
        60
    }
}

pub struct DuergarDwarf;
impl RaceFunctions for DuergarDwarf {
    fn darkvision(&self) -> usize {
        120
    }
}

pub struct HillDwarf;
impl RaceFunctions for HillDwarf {
    fn darkvision(&self) -> usize {
        60
    }
}

pub struct MountainDwarf;
impl RaceFunctions for MountainDwarf {
    fn darkvision(&self) -> usize {
        60
    }
}
