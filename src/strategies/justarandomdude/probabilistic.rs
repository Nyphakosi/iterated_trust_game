use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Probamimic(u32, u32); // count of shares, steals
impl fmt::Display for Probamimic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Probamimic")
    }
}
impl Strategy for Probamimic { // picks a random choice based on what opponent is probabilistically likely to pick
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if history.is_empty() {return true}
        if *history.last().unwrap_or(&true) {
            self.0 += 1
        } else {
            self.1 += 1
        }
        let shares = self.0 as f64;
        let steals = self.1 as f64;
        let proportion = shares / (shares+steals);
        rand::random_bool(proportion)
    }
}

#[derive(Clone)]
pub struct Predictor;
impl fmt::Display for Predictor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Predictor")
    }
}
impl Strategy for Predictor { // wip strat
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        unimplemented!()
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(Probamimic(0,0))]
}
