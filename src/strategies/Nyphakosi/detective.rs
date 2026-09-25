use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Detective;
impl fmt::Display for Detective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Detective")
    }
}
impl Strategy for Detective { // share, steal, share, share, if opponent steals back, act like copycat, else act like greedy
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if history.len() < 4 {return [true, false, true, true][history.len()]}
        if history[2] {
            false
        } else {
            *history.last().unwrap_or(&true)
        }
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(Detective)]
}
