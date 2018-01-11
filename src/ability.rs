
#[derive(Clone, Copy, PartialEq)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

pub trait Modifier {
    fn modifier(self) -> isize;
}

impl Modifier for usize {
    fn modifier(self) -> isize {
        let n = self as isize;
        if n < 10 {
            (n - 11) / 2
        } else {
            (n - 10) / 2
        }
    }
}
