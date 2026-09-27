use std::fmt;
use crate::Strategy;

#[derive(Clone)]
pub struct PredictorHC1([(u32, u32); 4]); // count of shares, steals, per situation
impl fmt::Display for PredictorHC1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PredictorHC1")
    }
}
impl Strategy for PredictorHC1 { 
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        if memory.len() < 2 {return true}
        let index: usize = (if  memory[ memory.len()-2] {1} else {0})
                         | (if history[history.len()-2] {1} else {0} << 1);
        let count: &mut (u32, u32) = &mut self.0[index];
        if history[history.len()-1] {count.0 += 1} else {count.1 +=1}
        let shares = count.0 as f64;
        let steals = count.1 as f64;
        let proportion = shares / (shares+steals);
        rand::random_bool(proportion)
    }
}

#[derive(Clone)]
pub struct PredictorHC2([(u32, u32); 16]); // count of shares, steals, per situation
impl fmt::Display for PredictorHC2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PredictorHC2")
    }
}
impl Strategy for PredictorHC2 { 
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        if memory.len() < 3 {return true}
        let index: usize = (if  memory[ memory.len()-2] {1} else {0})
                         | (if history[history.len()-2] {1} else {0} << 1)
                         | (if  memory[ memory.len()-3] {1} else {0} << 2)
                         | (if history[history.len()-3] {1} else {0} << 3);
        let count: &mut (u32, u32) = &mut self.0[index];
        if history[history.len()-1] {count.0 += 1} else {count.1 +=1}
        let shares = count.0 as f64;
        let steals = count.1 as f64;
        let proportion = shares / (shares+steals);
        rand::random_bool(proportion)
    }
}

#[derive(Clone)]
pub struct Predictor {
    running_count: Vec<(u32, u32)>, 
    window_len: usize
} // count of shares, steals, per situation, length of window
impl fmt::Display for Predictor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Predictor ({})", self.window_len)
    }
}
impl Strategy for Predictor { 
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        if memory.len() < self.window_len+1 {return true}
        let index: usize = {
            let mut index = 0;
            for i in 0..self.window_len {
                index += (if  memory[ memory.len()-2] {1} else {0} << (i*2))
                       | (if history[history.len()-2] {1} else {0} << (i*2 + 1))
            }
            index
        };
        let count: &mut (u32, u32) = &mut self.running_count[index];
        if history[history.len()-1] {count.0 += 1} else {count.1 +=1}
        false
    }
}
impl Predictor {
    fn make(n: usize) -> Self {
        Predictor {
            running_count: vec![(0,0);
                1 << (2*n)
            ],
            window_len: n
        }
    }
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        Box::new(PredictorHC1([(0,0);4])), Box::new(PredictorHC2([(0,0);16])), 
        Box::new(Predictor::make(1)), Box::new(Predictor::make(2)), Box::new(Predictor::make(3)), 
    ]
}
