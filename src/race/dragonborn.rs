
use race::RaceFunctions;

pub struct Dragonborn;

impl RaceFunctions for Dragonborn {
    fn base_ac(&self) -> usize {
        13
    }
}
