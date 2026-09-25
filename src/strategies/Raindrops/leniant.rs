use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct ForgivingGrudger(u32); // >=2 = grudging
impl fmt::Display for ForgivingGrudger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Forgiving Grudger")
    }
}
impl Strategy for ForgivingGrudger { // generous, unless opponent steals, then greedy
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if !*history.last().unwrap_or(&true) {self.0 += 1} // if opponent steals, switch to grudging state
        self.0 < 2
    }
}

#[derive(Clone)]
pub struct CautiousThankful(u32); // >=2 = thanking
impl fmt::Display for CautiousThankful {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cautious Thankful")
    }
}
impl Strategy for CautiousThankful { // greedy, unless opponent steals twice, then generous
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if *history.last().unwrap_or(&true) {self.0 += 1} // if opponent shares, switch to thanking state
        self.0 >= 2
    }
}

#[derive(Clone)]
pub struct Copykitten;
impl fmt::Display for Copykitten {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Copykitten")
    }
}
impl Strategy for Copykitten {// steals only if opponent steals twice in a row
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if history.len() < 2 {return true}
        history[history.len()-1] | history[history.len()-2]
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(ForgivingGrudger(0)), Box::new(CautiousThankful(0)), Box::new(Copykitten)]
}
