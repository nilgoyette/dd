
use race::{RaceFunctions, RaceSize};

pub struct Human;

impl RaceFunctions for Human {
    fn base_ac(&self) -> usize {
        10
    }

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
