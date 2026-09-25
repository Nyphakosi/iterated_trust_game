use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Copycat;
impl fmt::Display for Copycat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Copycat")
    }
}
impl Strategy for Copycat {// share, then opponent's last move
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        *history.last().unwrap_or(&true)
    }
}

#[derive(Clone)]
pub struct Anticat;
impl fmt::Display for Anticat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Anticat")
    }
}
impl Strategy for Anticat { // steal, then not opponent's last move
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        !*history.last().unwrap_or(&true)
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(Copycat), Box::new(Anticat)]
}
