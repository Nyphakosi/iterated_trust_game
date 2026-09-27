use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Pavlov;
impl fmt::Display for Pavlov {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pavlov")
    }
}
impl Strategy for Pavlov {// share, then opponent's last move
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        *memory.last().unwrap_or(&true) == *history.last().unwrap_or(&true)
    }
}

#[derive(Clone)]
pub struct Antipavlov;
impl fmt::Display for Antipavlov {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Antipavlov")
    }
}
impl Strategy for Antipavlov { // steal, then not opponent's last move
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        *memory.last().unwrap_or(&true) != *history.last().unwrap_or(&true)
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(Pavlov), Box::new(Antipavlov)]
}
