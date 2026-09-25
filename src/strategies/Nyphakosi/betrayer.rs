use std::fmt;
use crate::{ROUNDS, Strategy};


#[derive(Clone)]
pub struct Betrayer(f64); // percent of the game to share for
impl fmt::Display for Betrayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Betrayer ({})", self.0)
    }
}
impl Strategy for Betrayer { // generous for first proportion of the game, then switch to greedy
    fn decide(&mut self, memory: &[bool], _history: &[bool]) -> bool {
        memory.len() < (ROUNDS as f64 * self.0) as usize
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(Betrayer(0.25)), Box::new(Betrayer(0.50)), Box::new(Betrayer(0.75))]
}
