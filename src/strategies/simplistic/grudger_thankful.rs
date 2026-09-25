use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Grudger(bool); // false = grudging
impl fmt::Display for Grudger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grudger")
    }
}
impl Strategy for Grudger { // generous, unless opponent steals, then greedy
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if !*history.last().unwrap_or(&true) {self.0 = false} // if opponent steals, switch to grudging state
        self.0
    }
}

#[derive(Clone)]
pub struct Thankful(bool); // true = thanking
impl fmt::Display for Thankful {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Thankful")
    }
}
impl Strategy for Thankful { // greedy, unless opponent steals, then generous
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if *history.last().unwrap_or(&true) {self.0 = true} // if opponent shares, switch to thanking state
        self.0
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(Grudger(true)), Box::new(Thankful(false))]
}
