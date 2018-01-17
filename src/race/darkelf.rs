
use race::RaceFunctions;

pub struct DarkElf;

impl RaceFunctions for DarkElf {
    fn darkvision(&self) -> usize {
        120
    }
}
