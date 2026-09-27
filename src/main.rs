use std::thread;

use iterated_trust_game::*;
mod strategies;

// these are for stress testing the simulator
const DELAYTEST: bool = false; // inserts the DelayTest strategy, which is generous but with a timed delay to answering
const DELAYTEST_MS: u64 = 5; // ms per answer
const POOLTEST: bool = false; // inserts 2^POOLTEST_EXP Repecat strategies, which are just renamed Copycat to differentiate
const POOLTEST_EXP: usize = 8;
const INDEXTWO: bool = false; // whether or not to push 131070 unique strategies to the pool

//                (a,b)     a: steal  share      b:
// const TABLE: [(i32,i32); 4] = [(2,2), (2,8),  // steal
//                                (8,2), (6,5)]; // share
pub const TABLE: [(i32,i32); 4] = [(-1,-1), (-1, 3), 
                                   ( 3,-1), ( 2, 2)];
// const TABLE: [(i32,i32); 4] = [(-1,-1), (-1, 1), 
//                                ( 1,-1), ( 1, 1)];

pub const REPEAT_ROUNDS: u32 = 10;
pub const ROUNDS: u32 = 1000;
pub const COPIES: u32 = 1;
pub const MISPLAY_CHANCE: f64 = 0.05;

// when making a strategy
// return false: steal, return true: share
// memory is the previous moves the strategy has made, history is the moves the opponent has made
// strategies are allowed access to parameters of the match (like number of rounds) and access to mutable self state

fn main() {
    let _core_count = num_cpus::get();

    let strategytypes: Vec<Box<dyn Strategy>> = strategies::retrieve_strategies(); // collect strategies from subfolders and files
    let strategies = if COPIES != 1 {
        let mut temp = vec![];
        for strat in strategytypes {
            for _ in 0..COPIES {
                temp.push(dyn_clone::clone_box(&*strat))
            }
        }
        temp
    } else {strategytypes};
    let stratcount = strategies.len();
    let mut scores = vec![0; stratcount];

    // this is debug code for displaying a specific match
    // {
    //     let mut strat_a = dyn_clone::clone_box(&**strategies.iter().find(|x| format!("{}", x).contains(
    //         "Predictor (2)")).unwrap());
    //     let mut strat_b = dyn_clone::clone_box(&**strategies.iter().find(|x| format!("{}", x).contains(
    //         "Forgiving Grudger")).unwrap());
    //     _play_debug(&mut *strat_a, &mut *strat_b, 200, TABLE, 0.0);
    //     return;
    // }

    let timer = std::time::Instant::now();

    // let print_target = "Probamimic";
    println!("Playing Games...");

    // code from before multithreading attempt
    // for a in 0..strategies.len() { // play each strategy against each other, once
    //     for b in a..strategies.len() {
    //         let mut strat_a = dyn_clone::clone_box(&*strategies[a]);
    //         let mut strat_b = dyn_clone::clone_box(&*strategies[b]);
    //         let score = play(&mut *strat_a, &mut *strat_b, ROUNDS);
    //         scores[a] += score.0; scores[b] += score.1; // keep track of round scores
    //         // this is for seeing how a specific strategy does against all others
    //         // if format!("{}", strategies[a]) == print_target || format!("{}", strategies[b]) == print_target {
    //         //     println!("({:>5}, {:>5}) | {:?} vs {:?}", score.0, score.1, format!("{}", strategies[a]), format!("{}", strategies[b]));
    //         // }
    //     } 
    // }

    let balanced_table: bool = { // if the table is the same for both players
            TABLE[0].0 == TABLE[0].1
        &&  TABLE[1].0 == TABLE[2].1
        &&  TABLE[2].0 == TABLE[1].1
        &&  TABLE[3].0 == TABLE[3].1
    };

    let mut handles = vec![];
    for a in 0..strategies.len() { // play each strategy against each other, once
        println!("playing {}", strategies[a]);
        for _ in 0..REPEAT_ROUNDS {
            for b in (if balanced_table {a} else {0})..strategies.len() { // if unbalanced table, twice
                let mut strat_a = dyn_clone::clone_box(&*strategies[a]);
                let mut strat_b = dyn_clone::clone_box(&*strategies[b]);
                let handle = thread::spawn(move || {
                    //play_threaded(&strategies[a], &strategies[a], ROUNDS)
                    play(&mut *strat_a, &mut *strat_b, ROUNDS, TABLE, MISPLAY_CHANCE)
                });
                handles.push((handle, (a,b)));
            }
        }
    }
    for result in handles {
        match result.0.join() {
            Ok(v) => { // scores for strategies (a, b)
                scores[result.1.0] += v.0; scores[result.1.1] += v.1; // keep track of round scores
            },
            Err(e) => println!("Thread error {:?}", e),
        }
    }

    println!("Played games in {}ms", timer.elapsed().as_millis());

    let mut scoreboard: Vec<(String, i32)> = vec![];
    for i in 0..stratcount {
        scoreboard.push((format!("{}", strategies[i]), scores[i])) // associate strategy names with their scores
    }
    scoreboard.sort_by_key(|k| k.1);
    scoreboard.reverse();
    println!();
    println!("Scores for {}x{} rounds at {}% misplay chance", 
        REPEAT_ROUNDS, ROUNDS, MISPLAY_CHANCE*100.0, 
    );
    println!("With table:");
    println!("steal/steal:{}/{}, share/steal:{}/{}",
        TABLE[0].0, TABLE[0].1, TABLE[1].0, TABLE[1].1
    );
    println!("steal/share:{}/{}, share/share:{}/{}",
        TABLE[2].0, TABLE[2].1, TABLE[3].0, TABLE[3].1
    );
    for i in scoreboard.iter().enumerate() { // print the scoreboard
        println!("{:>4}: {:>7} | {:?}", i.0+1, i.1.1, i.1.0.to_string());
    }
}

