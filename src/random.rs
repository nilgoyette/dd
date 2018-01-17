
use std::ops::Add;

use rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Selections<T> {
    pub nb: usize,
    pub forced: Vec<T>,
    pub choices: Vec<T>
}

impl<T: Clone + PartialEq> Selections<T> {
    pub fn forced(forced: Vec<T>) -> Selections<T> {
        Selections { nb: 0, forced, choices: vec![] }
    }

    pub fn new(
        forced: Vec<T>,
        nb: usize,
        mut choices: Vec<T>
    ) -> Selections<T> {
        // A forced option can't be in choices
        for f in forced.iter() {
            if let Some(idx) = choices.iter().position(|e| *e == *f) {
                choices.remove(idx);
            }
        }
        Selections { nb, forced, choices }
    }

    pub fn auto_select(self) -> Vec<T> {
        if self.nb == self.choices.len() {
            let mut selections = self.forced.clone();
            selections.extend(self.choices);
            return selections;
        }

        let mut rng = rand::thread_rng();
        if self.nb == 1 {
            let mut selections = self.forced.clone();
            selections.push(rng.choose(&self.choices).unwrap().clone());
            return selections;
        }

        let mut all_choices = self.choices.clone();
        rng.shuffle(&mut all_choices);

        let mut selections = self.forced.clone();
        selections.extend(all_choices[..self.nb].to_vec());
        selections
    }
}

impl<T: Clone + PartialEq> Add for Selections<T> {
    type Output = Selections<T>;

    fn add(self, rhs: Selections<T>) -> Selections<T> {
        let mut forced = self.forced.clone();
        for e in rhs.forced {
            if !forced.contains(&e) {
                forced.push(e);
            }
        }

        let mut choices = self.choices.clone();
        for e in rhs.choices {
            if !choices.contains(&e) {
                choices.push(e);
            }
        }

        Selections::new(forced, self.nb + rhs.nb, choices)
    }
}

pub fn d6() -> usize {
    let mut rng = rand::thread_rng();
    let between = Range::new(1, 6);
    between.ind_sample(&mut rng)
}

pub fn d8() -> usize {
    let mut rng = rand::thread_rng();
    let between = Range::new(1, 8);
    between.ind_sample(&mut rng)
}

pub fn d10() -> usize {
    let mut rng = rand::thread_rng();
    let between = Range::new(1, 10);
    between.ind_sample(&mut rng)
}

pub fn d12() -> usize {
    let mut rng = rand::thread_rng();
    let between = Range::new(1, 12);
    between.ind_sample(&mut rng)
}

pub fn d20() -> usize {
    let mut rng = rand::thread_rng();
    let between = Range::new(1, 20);
    between.ind_sample(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selections_add() {
        let s1 = Selections::new(vec![1, 2], 1, vec![10, 11, 12]);

        let s = s1.clone() + Selections::forced(vec![2, 3, 9]);
        assert_eq!(s.forced, vec![1, 2, 3, 9]);
        assert_eq!(s.nb, 1);
        assert_eq!(s.choices, vec![10, 11, 12]);

        let s = s1.clone() + Selections::new(vec![2, 10], 2, vec![12, 13]);
        assert_eq!(s.forced, vec![1, 2, 10]);
        assert_eq!(s.nb, 3);
        assert_eq!(s.choices, vec![11, 12, 13]);
        assert_eq!(s.auto_select(), vec![1, 2, 10, 11, 12, 13]);

        // Same test but with a different ordering
        let s = s1.clone() + Selections::new(vec![10, 2], 2, vec![13, 12]);
        assert_eq!(s.forced, vec![1, 2, 10]);
        assert_eq!(s.nb, 3);
        assert_eq!(s.choices, vec![11, 12, 13]);
        assert_eq!(s.auto_select(), vec![1, 2, 10, 11, 12, 13]);
    }
}
