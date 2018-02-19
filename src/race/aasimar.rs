
use race::{RaceFunctions, RaceSize};

pub struct Aasimar;

impl RaceFunctions for Aasimar {
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
