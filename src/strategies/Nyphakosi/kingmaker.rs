use std::fmt;
use crate::Strategy;

// const CULT_LEADER_KEY: [bool; 8] = [true, false, false, true, true, false, true, false];
// const CULTIST_KEY: [bool; 8] = [false, true, true, false, false, true, false, true];
const PEASANT_KEY: [bool; 8] = [false, true, false, true, true, true, false, false];


#[derive(Clone)]
pub struct Peasant(u8);
impl fmt::Display for Peasant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Peasant {}", self.0)
    }
}
impl Strategy for Peasant { // if paired against Peasant, become generous, else become greedy
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if history.len() < PEASANT_KEY.len() {return PEASANT_KEY[history.len()]}
        ({
            let mut sum_ones = 0;
            for i in 0..PEASANT_KEY.len() {
                if history[i] ^ PEASANT_KEY[i] {sum_ones += 1}
            }
            sum_ones
        } < 2) // if key matches all but one flipped bit, still allow it
    }
}

#[derive(Clone)]
pub struct King;
impl fmt::Display for King {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "King")
    }
}
impl Strategy for King { // steal from them peasants, else act like copycat
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        if history.len() < PEASANT_KEY.len() {return PEASANT_KEY[history.len()]}
        if history[0..PEASANT_KEY.len()] == PEASANT_KEY {return false}
        *history.last().unwrap_or(&true)
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    let mut v: Vec<Box<dyn Strategy>> = vec![
        Box::new(King), 
    ];
    for i in 0..(1<<2) {v.push(Box::new(Peasant(i)))}
    v
}
