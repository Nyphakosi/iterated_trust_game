use std::{collections::HashMap, fmt};
use crate::Strategy;

fn make_index(bak: usize, len: usize, memory: &[bool], history: &[bool]) -> usize {
    // creates an index based on the binary choices of previous moves, for arbitrary n moves back
    let mut index: usize = 0;
    for i in 0..len {
        index |= (if  memory[ memory.len()-(i+1+bak)] {1} else {0} << (i*2))
               | (if history[history.len()-(i+1+bak)] {1} else {0} << (i*2 + 1))
    }
    index
}

#[derive(Clone)]
struct PredictorHC1([(u32, u32); 4]); // count of shares, steals, per situation
impl fmt::Display for PredictorHC1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PredictorHC1")
    }
}
impl Strategy for PredictorHC1 {  // hardcoded 1-turn memory
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        if memory.len() < 2 {return true}
        // let index: usize = (if  memory[ memory.len()-2] {1} else {0})
        //                  | (if history[history.len()-2] {1} else {0} << 1);
        let index = make_index(1, 1, memory, history);
        let count: &mut (u32, u32) = &mut self.0[index];
        if history[history.len()-1] {count.0 += 1} else {count.1 +=1}
        let index = make_index(0, 1, memory, history);
        let count: &mut (u32, u32) = &mut self.0[index];
        let shares = count.0 as f64;
        let steals = count.1 as f64;
        let proportion = shares / (shares+steals);
        if !proportion.is_nan() {rand::random_bool(proportion)} else {true}
    }
}

#[derive(Clone)]
struct PredictorHC2([(u32, u32); 16]); // count of shares, steals, per situation
impl fmt::Display for PredictorHC2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PredictorHC2")
    }
}
impl Strategy for PredictorHC2 { // hardcoded 2-turn memory
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        if memory.len() < 3 {return true}
        // let index_prev: usize = (if  memory[ memory.len()-2] {1} else {0})
        //                       | (if history[history.len()-2] {1} else {0} << 1)
        //                       | (if  memory[ memory.len()-3] {1} else {0} << 2)
        //                       | (if history[history.len()-3] {1} else {0} << 3);
        let index = make_index(1, 2, memory, history);
        let count: &mut (u32, u32) = &mut self.0[index];
        if history[history.len()-1] {count.0 += 1} else {count.1 +=1}
        // let index_next: usize = (if  memory[ memory.len()-1] {1} else {0})
        //                       | (if history[history.len()-1] {1} else {0} << 1)
        //                       | (if  memory[ memory.len()-2] {1} else {0} << 2)
        //                       | (if history[history.len()-2] {1} else {0} << 3);
        let index = make_index(0, 2, memory, history);
        let count: &(u32, u32) = &self.0[index];
        let shares = count.0 as f64;
        let steals = count.1 as f64;
        let proportion = shares / (shares+steals);
        if !proportion.is_nan() {rand::random_bool(proportion)} else {true}
    }
}

#[derive(Clone)]
struct Predictor {
    running_count: Vec<(u32, u32)>, // count of shares, steals, per situation
    window_len: usize
}
impl fmt::Display for Predictor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Predictor ({})", self.window_len)
    }
}
impl Strategy for Predictor { // predicts what move the opponent will make this turn based on past data
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        // random until data collecting window length
        if memory.len() < self.window_len+1 {return rand::random_bool(0.5)}
        // update the index for the previous case
        let index: usize = make_index(1, self.window_len, memory, history);
        let count: &mut (u32, u32) = &mut self.running_count[index];
        if history[history.len()-1] {count.0 += 1} else {count.1 +=1}
        // retrieve the data for the current case
        let index: usize = make_index(0, self.window_len, memory, history);
        let count: &mut (u32, u32) = &mut self.running_count[index];
        // calculate proportion, share if nan
        let shares = count.0 as f64;
        let steals = count.1 as f64;
        let proportion = shares / (shares+steals);
        let proportion = if proportion.is_nan() {0.5} else {proportion};
        rand::random_bool(proportion)
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

// #[derive(Clone)]
// struct Predictor {
//     running_count: HashMap<usize, (u32, u32)>, // count of shares, steals, per situation
//     window_len: usize
// }
// impl fmt::Display for Predictor {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Predictor ({})", self.window_len)
//     }
// }
// impl Strategy for Predictor { // predicts what move the opponent will make this turn based on past data
//     fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
//         // update the index for the previous case
//         if memory.len() < self.window_len+1 {return true}
//         let index: usize = make_index(1, self.window_len, memory, history);
//         let count = self.running_count.entry(index).or_insert((0, 0));
//         if history[history.len()-1] {count.0 += 1} else {count.1 +=1}
//         // retrieve the data for the current case
//         let index: usize = make_index(0, self.window_len, memory, history);
//         // calculate proportion, share if nan
//         self.running_count.get(&index)
//             .map(|(shares, steals)|
//                  rand::random_bool(*shares as f64 / (shares + steals) as f64)
//             )
//             .unwrap_or(rand::random_bool(0.5))
//     }
// }
// impl Predictor {
//     fn make(n: usize) -> Self {
//         Predictor {
//             running_count: HashMap::new(),
//             window_len: n
//         }
//     }
// }

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    // vec![
    //     //Box::new(PredictorHC1([(0,0);4])), Box::new(PredictorHC2([(0,0);16])), 
    //     Box::new(Predictor::make(1)), Box::new(Predictor::make(2)), Box::new(Predictor::make(3)), 
    // ]
    let mut v: Vec<Box<dyn Strategy>> = vec![ ];
    for i in 1..5 {v.push(Box::new(Predictor::make(i)))}
    v
}
