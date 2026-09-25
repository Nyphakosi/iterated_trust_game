use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct Index(u8); // index id from 0 to 31
impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index ({})", self.0)
    }
}
impl Strategy for Index { // always steals
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        if memory.is_empty() {return self.0 & 0b10000 != 0}
        match (memory.last().unwrap_or(&true), history.last().unwrap_or(&true)) {
            (false, false) => self.0 & 0b00001 != 0,
            (false,  true) => self.0 & 0b00010 != 0,
            ( true, false) => self.0 & 0b00100 != 0,
            ( true,  true) => self.0 & 0b01000 != 0,
        }
    }
}

fn todo() {} // make this better
pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    vec![
                      &Index(1),  &Index(2),  &Index(3),  &Index(4),  &Index(5),  &Index(6),  &Index(7), 
         &Index(8),   &Index(9), &Index(10), &Index(11), &Index(12), &Index(13), &Index(14), &Index(15), 
        &Index(16),  &Index(17), &Index(18), &Index(19), &Index(20), &Index(21), &Index(22), &Index(23), 
        &Index(24),  &Index(25), &Index(26), &Index(27), &Index(28), &Index(29), &Index(30), 
    ]
}
