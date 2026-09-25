use std::fmt;
use crate::Strategy;


#[derive(Clone)]
pub struct Random(f64); // chance to share
impl fmt::Display for Random {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Random ({})", self.0)
    }
}
impl Strategy for Random { // pick randomly
    fn decide(&mut self, _memory: &[bool], _history: &[bool]) -> bool {
        rand::random_bool(self.0)
    }
}

pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    vec![&Random(0.25), &Random(0.50), &Random(0.75)]
}
