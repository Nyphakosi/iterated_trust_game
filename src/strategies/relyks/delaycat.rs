use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Delaycat(usize); // how many turns back to copy
impl fmt::Display for Delaycat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Delaycat")
    }
}
impl Strategy for Delaycat { // share, then opponent's move n turns ago
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if history.len() < self.0 {return true}
        history[history.len()-self.0]
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    let mut v: Vec<Box<dyn Strategy>> = vec![];
    for i in 1..4 {
        v.push(Box::new(Delaycat(1<<i)))
    }
    v
}
