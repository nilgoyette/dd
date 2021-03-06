
use std::ops::Add;

use rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Selections<T> {
    pub forced: Vec<T>,
    pub nb_choices: usize,
    pub choices: Vec<T>
}

impl<T: Clone + PartialEq> Selections<T> {
    pub fn forced(forced: Vec<T>) -> Selections<T> {
        Selections { forced, nb_choices: 0, choices: vec![] }
    }

    pub fn choices(nb_choices: usize, choices: Vec<T>) -> Selections<T> {
        Selections { forced: vec![], nb_choices, choices }
    }

    pub fn new(
        forced: Vec<T>,
        nb_choices: usize,
        mut choices: Vec<T>
    ) -> Selections<T> {
        // A forced option can't be in choices
        for f in forced.iter() {
            if let Some(idx) = choices.iter().position(|e| *e == *f) {
                choices.remove(idx);
            }
        }
        Selections { forced, nb_choices, choices }
    }

    pub fn add_forced(&mut self, new_item: T) {
        //Don't add it if it's already there
        if self.forced.iter().find(|&x| *x == new_item).is_none() {
            self.forced.push(new_item.clone());

            // A forced option can't be in choices
            if let Some(idx) = self.choices.iter().position(
                |e| *e == new_item)
            {
                self.choices.remove(idx);
            }
        }
    }

    pub fn add_choice(&mut self, new_item: T) {
        // Only add if it's not in forced, nor in choices
        if self.forced.iter().find(|&x| *x == new_item).is_none() {
            if self.choices.iter().find(|&x| *x == new_item).is_none() {
                self.choices.push(new_item);
            }
        }
    }

    pub fn auto_select(self) -> Vec<T> {
        if self.nb_choices == self.choices.len() {
            let mut selections = self.forced;
            selections.extend(self.choices);
            return selections;
        }

        let mut rng = rand::thread_rng();
        if self.nb_choices == 1 {
            let mut selections = self.forced;
            selections.push(rng.choose(&self.choices).unwrap().clone());
            return selections;
        }

        let mut all_choices = self.choices;
        rng.shuffle(&mut all_choices);

        let mut selections = self.forced;
        selections.extend(all_choices[..self.nb_choices].to_vec());
        selections
    }
}

impl<T: Clone + PartialEq> Add for Selections<T> {
    type Output = Selections<T>;

    fn add(self, rhs: Selections<T>) -> Selections<T> {
        let mut forced = self.forced;
        for e in rhs.forced {
            if !forced.contains(&e) {
                forced.push(e);
            }
        }

        let mut choices = self.choices;
        for e in rhs.choices {
            if !choices.contains(&e) {
                choices.push(e);
            }
        }

        Selections::new(forced, self.nb_choices + rhs.nb_choices, choices)
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
        assert_eq!(s.nb_choices, 1);
        assert_eq!(s.choices, vec![10, 11, 12]);

        let s = s1.clone() + Selections::new(vec![2, 10], 2, vec![12, 13]);
        assert_eq!(s.forced, vec![1, 2, 10]);
        assert_eq!(s.nb_choices, 3);
        assert_eq!(s.choices, vec![11, 12, 13]);
        assert_eq!(s.auto_select(), vec![1, 2, 10, 11, 12, 13]);

        // Same test but with a different ordering
        let s = s1.clone() + Selections::new(vec![10, 2], 2, vec![13, 12]);
        assert_eq!(s.forced, vec![1, 2, 10]);
        assert_eq!(s.nb_choices, 3);
        assert_eq!(s.choices, vec![11, 12, 13]);
        assert_eq!(s.auto_select(), vec![1, 2, 10, 11, 12, 13]);
    }

    #[test]
    fn test_selections_update() {
        let mut s = Selections::new(vec![1, 2], 1, vec![10, 11, 12]);
        for n in 1..4 {
            s.add_forced(n);
        }
        assert_eq!(s.forced, vec![1, 2, 3]);
        assert_eq!(s.choices, vec![10, 11, 12]);

        // Should remove in choices
        s.add_forced(10);
        assert_eq!(s.forced, vec![1, 2, 3, 10]);
        assert_eq!(s.choices, vec![11, 12]);

        // Should do nothing
        s.add_choice(1);
        s.add_choice(12);
        assert_eq!(s.forced, vec![1, 2, 3, 10]);
        assert_eq!(s.choices, vec![11, 12]);

        // Should add a choice
        s.add_choice(13);
        assert_eq!(s.forced, vec![1, 2, 3, 10]);
        assert_eq!(s.choices, vec![11, 12, 13]);
    }
}
