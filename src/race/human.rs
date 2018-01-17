
use race::RaceFunctions;

pub struct Human;

impl RaceFunctions for Human {
    fn darkvision(&self) -> usize {
        0
    }
}
