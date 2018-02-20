
use random::{d6, d8, d10, d12};
use Modifier;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HitDice {
    D6, D8, D10, D12
}

impl HitDice {
    pub fn max(&self) -> usize {
        match *self {
            HitDice::D6 => 6,
            HitDice::D8 => 8,
            HitDice::D10 => 10,
            HitDice::D12 => 12
        }
    }

    pub fn first_level(&self, constitution: usize) -> usize {
        (self.max() as isize + constitution.modifier()) as usize
    }

    pub fn level_up(&self, constitution: usize) -> (usize, usize) {
        let (random_dice, safe_choice) = match *self {
            HitDice::D6 => (d6(), 4),
            HitDice::D8 => (d8(), 5),
            HitDice::D10 => (d10(), 6),
            HitDice::D12 => (d12(), 7)
        };
        let bonus = constitution.modifier();
        let random_dice = (random_dice as isize + bonus).max(1);
        let safe_choice = (safe_choice as isize + bonus).max(1);
        (random_dice as usize, safe_choice as usize)
    }
}
