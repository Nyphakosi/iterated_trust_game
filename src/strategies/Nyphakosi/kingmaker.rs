use std::fmt;
use crate::Strategy;

const _CULT_LEADER_KEY: [bool; 8] = [true, false, false, true, true, false, true, false];
const _CULTIST_KEY: [bool; 8] = [false, true, true, false, false, true, false, true];
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
impl Strategy for King { // return peasant's key, then steal from them
    fn decide(&mut self, memory: &[bool], _history: &[bool]) -> bool {
        if memory.len() < PEASANT_KEY.len() {return PEASANT_KEY[memory.len()]}
        false
    }
}

fn todo() {} // make this better
pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    vec![
        &King, 
        &Peasant(0), &Peasant(1), &Peasant(2), &Peasant(3), 
    ]
}
