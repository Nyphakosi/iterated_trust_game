use dyn_clone::{self, DynClone};
mod strategies;

//                (a,b)     a: steal  share      b:
// const TABLE: [(i32,i32); 4] = [(2,2), (2,8),  // steal
//                                (8,2), (5,5)]; // share
const TABLE: [(i32,i32); 4] = [(-1,-1), (-1,3), 
                               (3,-1), (2,2)];

pub const ROUNDS: u32 = 1000;
const _COPIES: u32 = 1;
const MISPLAY_CHANCE: f64 = 0.05;

//type Strategy = fn(&[bool], &[bool]) -> bool;

trait Strategy: std::fmt::Display + DynClone {
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool;
}

#[macro_export]
macro_rules! retrieve_strategies {
    ($($name:ident),*) => {
        $(mod $name;)*

        pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
            let mut v = vec![];
            $(v.append(&mut $name::retrieve_strategies());)*
            v
        }
    };
}

fn main() {
    // return true: share
    // return false: steal

    let strategies: Vec<Box<dyn Strategy>> = strategies::retrieve_strategies();
    let stratcount = strategies.len();
    let mut scores = vec![0; stratcount];

    // let mut strat_a = dyn_clone::clone_box(&**strategies.iter().find(|x| format!("{}", x).contains(
    //     "Probamimic")).unwrap());
    // let mut strat_b = dyn_clone::clone_box(&**strategies.iter().find(|x| format!("{}", x).contains(
    //     "Businessman")).unwrap());
    // _play_debug(&mut *strat_a, &mut *strat_b, 200);

    // let print_target = "Probamimic";
    println!("Playing Games...");
    for a in 0..strategies.len() {
        for b in a..strategies.len() {
            let mut strat_a = dyn_clone::clone_box(&*strategies[a]);
            let mut strat_b = dyn_clone::clone_box(&*strategies[b]);
            let score = play(&mut *strat_a, &mut *strat_b, ROUNDS);
            scores[a] += score.0; scores[b] += score.1;
            // if format!("{}", strategies[a]) == print_target || format!("{}", strategies[b]) == print_target {
            //     println!("({:>5}, {:>5}) | {:?} vs {:?}", score.0, score.1, format!("{}", strategies[a]), format!("{}", strategies[b]));
            // }
        } 
    }

    let mut scoreboard: Vec<(String, i32)> = vec![];
    for i in 0..stratcount {
        scoreboard.push((format!("{}", strategies[i]), scores[i]))
    }
    scoreboard.sort_by_key(|k| k.1);
    scoreboard.reverse();
    println!();
    println!("Scores for {} rounds at {}% misplay chance, with table ", 
        ROUNDS, MISPLAY_CHANCE*100.0, 
    );
    println!("With table steal/steal:{}/{}, steal/share:{}/{}, share/share:{}/{}",
        TABLE[0].0, TABLE[0].1, TABLE[2].0, TABLE[2].1, TABLE[3].0, TABLE[3].1,
    );
    for i in scoreboard.iter().enumerate() {
        println!("{:>3}: {:>6} | {:?}", i.0+1, i.1.1, i.1.0.to_string());
    }
}

fn play(a: &mut dyn Strategy, b: &mut dyn Strategy, rounds: u32) -> (i32, i32) { // returns total points
    let mut moves_a = vec![]; // previous moves a has made
    let mut moves_b = vec![]; // previous moves b has made
    let mut sum_score = (0, 0); // total score
    for _round in 0..rounds { // play for n rounds
        let move_a = a.decide(&moves_a, &moves_b) ^ rand::random_bool(MISPLAY_CHANCE);
        let move_b = b.decide(&moves_b, &moves_a) ^ rand::random_bool(MISPLAY_CHANCE);
        let round_score = TABLE[if move_a {1} else {0} | if move_b {2} else {0}]; // will never exceed 3
        sum_score = (sum_score.0 + round_score.0, sum_score.1 + round_score.1);
        moves_a.push(move_a);
        moves_b.push(move_b);
    }
    sum_score
}

fn _play_debug(a: &mut dyn Strategy, b: &mut dyn Strategy, rounds: u32) -> (i32, i32) { // returns total points
    println!("{} vs {}", a, b);
    let mut moves_a = vec![]; // previous moves a has made
    let mut moves_b = vec![]; // previous moves b has made
    let mut sum_score = (0, 0); // total score
    for _round in 0..rounds { // play for n rounds
        let move_a = a.decide(&moves_a, &moves_b) ^ rand::random_bool(MISPLAY_CHANCE);
        let move_b = b.decide(&moves_b, &moves_a) ^ rand::random_bool(MISPLAY_CHANCE);
        println!("{move_a:>5}, {move_b:>5}");
        let round_score = TABLE[if move_a {1} else {0} | if move_b {2} else {0}]; // will never exceed 3
        sum_score = (sum_score.0 + round_score.0, sum_score.1 + round_score.1);
        moves_a.push(move_a);
        moves_b.push(move_b);
    }
    println!("({}, {})", sum_score.0, sum_score.1);
    sum_score
}