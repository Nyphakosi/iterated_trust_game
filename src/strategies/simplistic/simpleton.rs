use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Simpleton;
impl fmt::Display for Simpleton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simpleton")
    }
}
impl Strategy for Simpleton { // start with share, repeat last move, unless opponent steals, then swap
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        *memory.last().unwrap_or(&true) ^ !*history.last().unwrap_or(&true)
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(Simpleton)]
}
