
use rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Choice<T> {
    pub nb: usize,
    pub choices: Vec<T>
}

impl<T: Clone> Choice<T> {
    pub fn new(nb: usize, choices: Vec<T>) -> Choice<T> {
        Choice { nb, choices }
    }

    pub fn choose(self) -> Vec<T> {
        if self.nb == self.choices.len() {
            return self.choices;
        }

        let mut rng = rand::thread_rng();
        if self.nb == 1 {
            return vec![rng.choose(&self.choices).unwrap().clone()];
        }

        let mut all_choices = self.choices.clone();
        rng.shuffle(&mut all_choices);
        all_choices[..self.nb].to_vec()
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
