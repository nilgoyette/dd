
use race::RaceFunctions;

pub struct Aasimar;

impl RaceFunctions for Aasimar {
    fn darkvision(&self) -> usize {
        60
    }
}
