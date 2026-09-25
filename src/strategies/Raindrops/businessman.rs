use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Businessman;
impl fmt::Display for Businessman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Businessman")
    }
}
impl Strategy for Businessman { // not yet implemented, acts like generous
    fn decide(&mut self, _memory: &[bool], _history: &[bool]) -> bool {
        true
    }
}

pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    vec![]
}
