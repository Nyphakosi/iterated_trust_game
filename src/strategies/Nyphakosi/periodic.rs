use std::{fmt, thread, time};
use crate::Strategy;

#[derive(Clone)]
pub struct Periodic(usize, u32); // period, sequence of moves as a binary string
impl fmt::Display for Periodic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = self.0; let b = self.1;
        write!(f, "Periodic ({b:0p$b})")
    }
}
impl Strategy for Periodic { // repeat a sequence of moves
    fn decide(&mut self, memory: &[bool], _history: &[bool]) -> bool {
        let p = self.0; let b = self.1;
        b & (1<<(p - (memory.len() % p)-1)) != 0 // pick bit from binary num by logical AND with leftshifted 1
    }
}

#[derive(Clone)]
pub struct DelayTest;
impl fmt::Display for DelayTest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DelayTest")
    }
}
impl Strategy for DelayTest { // generous, but delayed to test multithreading
    fn decide(&mut self, _memory: &[bool], _history: &[bool]) -> bool {
        thread::sleep(time::Duration::from_millis(1));
        true
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    // vec![
    //     &Periodic(2, 0b01), &Periodic(2, 0b10), 
    //     &Periodic(3, 0b001), &Periodic(3, 0b010), &Periodic(3, 0b011), &Periodic(3, 0b100), &Periodic(3, 0b101), &Periodic(3, 0b110), 
    //     &Periodic(4, 0b0001), &Periodic(4, 0b0010), &Periodic(4, 0b0011), &Periodic(4, 0b0100), &Periodic(4, 0b0101), &Periodic(4, 0b0110), &Periodic(4, 0b0111), 
    //     &Periodic(4, 0b1000), &Periodic(4, 0b1001), &Periodic(4, 0b1010), &Periodic(4, 0b1011), &Periodic(4, 0b1100), &Periodic(4, 0b1101), &Periodic(4, 0b1110), 
    // ]
    let mut v: Vec<Box<dyn Strategy>> = vec![];
    for period in 2..=4 {
        for sequence in 1..((1<<period)-1) {
            v.push(Box::new(Periodic(period, sequence as u32)))
        }
    }
    //v.push(Box::new(DelayTest));
    v
}
