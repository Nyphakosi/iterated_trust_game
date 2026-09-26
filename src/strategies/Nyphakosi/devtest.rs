use std::{fmt, thread, time};
use crate::*;

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

#[derive(Clone)]
pub struct Repecat;
impl fmt::Display for Repecat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Repecat")
    }
}
impl Strategy for Repecat { // share, then opponent's last move, different name to distinguish from original copycat
    fn decide(&mut self, _memory: &[bool], history: &[bool]) -> bool {
        *history.last().unwrap_or(&true)
    }
}


pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    let mut v: Vec<Box<dyn Strategy>> = vec![];
    if DELAYTEST {v.push(Box::new(DelayTest))}
    if POOLTEST{
        for _ in 0..(1<<POOLTEST_EXP) { // push a fuckload of repecats to the strategy pool
            v.push(Box::new(Repecat))
        }
    }
    v
}
