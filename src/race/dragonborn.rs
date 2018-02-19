
use race::{RaceFunctions, RaceSize};

pub struct Dragonborn;

impl RaceFunctions for Dragonborn {
    fn darkvision(&self) -> usize {
        0
    }

    fn size(&self) -> RaceSize {
        RaceSize::Medium
    }

    fn speed(&self) -> usize {
        30
    }
}
