use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Greedy;
impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}
impl Strategy for Greedy { // always steals
    fn decide(&mut self, _memory: &[bool], _history: &[bool]) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct Generous;
impl fmt::Display for Generous {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generous")
    }
}
impl Strategy for Generous { // always shares
    fn decide(&mut self, _memory: &[bool], _history: &[bool]) -> bool {
        true
    }
}

pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    vec![&Greedy, &Generous]
}
