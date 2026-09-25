use std::fmt;
use crate::Strategy;


#[derive(Clone)]
pub struct Chancecat(f64, f64); // chance to play like copycat, chance to share if not
impl fmt::Display for Chancecat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chancecat ({}, {})", self.0, self.1)
    }
}
impl Strategy for Chancecat { 
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if rand::random_bool(self.0) {
            *history.last().unwrap_or(&true)
        } else {
            rand::random_bool(self.1)
        }
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(Chancecat(0.25, 0.50)), Box::new(Chancecat(0.50, 0.50)), Box::new(Chancecat(0.75, 0.50))]
}
