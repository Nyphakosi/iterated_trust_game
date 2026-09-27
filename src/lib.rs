use std::thread::*;
use std::sync::{*, mpsc::{Sender as Tx, Receiver as Rx}};

use dyn_clone::{self, DynClone};

pub trait Strategy: std::fmt::Display + DynClone + Send + Sync {
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool;
}

#[macro_export]
macro_rules! retrieve_strategies { // get strategies from subfolders/files
    ($($name:ident),*) => {
        $(mod $name;)*

        pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
            let mut v = vec![];
            $(v.append(&mut $name::retrieve_strategies());)*
            v
        }
    };
}

// returns total points
pub fn play(a: &mut dyn Strategy, b: &mut dyn Strategy, rounds: u32, table: [(i32,i32);4], misplay_chance: f64) -> (i32, i32) { 
    let mut moves_a = vec![]; // previous moves a has made
    let mut moves_b = vec![]; // previous moves b has made
    let mut sum_score = (0, 0); // total score
    for _round in 0..rounds { // play for n rounds
        let move_a = a.decide(&moves_a, &moves_b) ^ rand::random_bool(misplay_chance);
        let move_b = b.decide(&moves_b, &moves_a) ^ rand::random_bool(misplay_chance);
        let round_score = table[if move_a {1} else {0} | if move_b {2} else {0}]; // will never exceed 3
        sum_score = (sum_score.0 + round_score.0, sum_score.1 + round_score.1);
        moves_a.push(move_a);
        moves_b.push(move_b);
    }
    sum_score
}

// fn play_threaded(a: &mut dyn Strategy, b: &mut dyn Strategy, rounds: u32, table: [(i32,i32);4], misplay_chance: f64) -> (i32, i32) {
//     let mut strat_a = dyn_clone::clone_box(&*a);
//     let mut strat_b = dyn_clone::clone_box(&*b);
//     let handle = thread::spawn(move || {
//         let mut moves_a = vec![]; // previous moves a has made
//         let mut moves_b = vec![]; // previous moves b has made
//         let mut sum_score = (0, 0); // total score
//         for _round in 0..rounds { // play for n rounds
//             let move_a = strat_a.decide(&moves_a, &moves_b) ^ rand::random_bool(misplay_chance);
//             let move_b = strat_b.decide(&moves_b, &moves_a) ^ rand::random_bool(misplay_chance);
//             let round_score = table[if move_a {1} else {0} | if move_b {2} else {0}]; // will never exceed 3
//             sum_score = (sum_score.0 + round_score.0, sum_score.1 + round_score.1);
//             moves_a.push(move_a);
//             moves_b.push(move_b);
//         }
//         sum_score
//     });
//     match handle.join() {
//         Ok(s) => return s,
//         Err(e) => panic!("Thread error: {:?}", e),
//     }
// }

// returns total points
// prints every move
pub fn _play_debug(a: &mut dyn Strategy, b: &mut dyn Strategy, rounds: u32, table: [(i32,i32);4], misplay_chance: f64) -> (i32, i32) {
    println!("{} vs {}", a, b);
    let mut moves_a = vec![]; // previous moves a has made
    let mut moves_b = vec![]; // previous moves b has made
    let mut sum_score = (0, 0); // total score
    for _round in 0..rounds { // play for n rounds
        let move_a = a.decide(&moves_a, &moves_b) ^ rand::random_bool(misplay_chance);
        let move_b = b.decide(&moves_b, &moves_a) ^ rand::random_bool(misplay_chance);
        println!("{move_a:>5}, {move_b:>5}");
        let round_score = table[if move_a {1} else {0} | if move_b {2} else {0}]; // will never exceed 3
        sum_score = (sum_score.0 + round_score.0, sum_score.1 + round_score.1);
        moves_a.push(move_a);
        moves_b.push(move_b);
    }
    println!("({}, {})", sum_score.0, sum_score.1);
    sum_score
}

#[allow(dead_code)]
type Amrx<T> = Arc<Mutex<Rx<T>>>;
#[allow(dead_code)]
fn thrpool<T: Send, U: Send, W: Send, R>( // i have no idea how this function works
    nthr: usize,
    worker: impl Sync + Fn(Amrx<T>, Tx<U>) -> W,
    manager: impl for<'scope>
        FnOnce(Tx<T>, Rx<U>, Box<[ScopedJoinHandle<'scope, W>]>) -> R,
) -> R {
    let worker = &worker;
    std::thread::scope(|scope| {
        let (task_tx, task_rx) = std::sync::mpsc::channel();
        let (res_tx, res_rx) = std::sync::mpsc::channel();
        let task_rx = Arc::new(Mutex::new(task_rx));
        let handles: Box<[_]> = (0..nthr).map(|_| {
            let (task_rx, res_tx) = (Arc::clone(&task_rx), res_tx.clone());
            scope.spawn(move || worker(task_rx, res_tx))
        }).collect();
        drop((task_rx, res_tx));
        manager(task_tx, res_rx, handles)
    })
}