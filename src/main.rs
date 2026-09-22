use rand::{self, seq::IndexedRandom};

//                (a,b)     a: steal  share      b:
// const TABLE: [(i32,i32); 4] = [(2,2), (2,8),  // steal
//                                (8,2), (5,5)]; // share
const TABLE: [(i32,i32); 4] = [(0,0), (-1,3), 
                               (3,-1), (2,2)];

type Stratfn = fn(&[bool], &[bool]) -> bool;

enum Strategy {
    Generous,
    Greedy,
    Grudger,
    Thankful,
    Mimic,
    Antimimic,
    Alternating,
    Random,
    Tester,
}

impl Strategy {
    fn decide(&self, mymoves: &[bool], oppmoves: &[bool]) -> bool {
        use Strategy::*;
        match self {
            Generous => true,
            Greedy => false,
            Grudger => !oppmoves.contains(&false),
            Thankful => oppmoves.contains(&true),
            Mimic => if oppmoves.is_empty() {true} else {oppmoves[oppmoves.len()-1]},
            Antimimic => match oppmoves.last() {None => false, Some(x) => !*x},
            Alternating => match mymoves.last() {None => true, Some(x) => !*x},
            Random => match [true, false].choose(&mut rand::rng()) {None => unreachable!(), Some(x) => *x},
            Tester => false,
        }
    }
}

fn main() {
    // return true: share
    // return false: steal
    // strategies
    let generous = |_mymoves: &[bool], _oppmoves: &[bool]| -> bool {true}; // always share
    let greedy = |_mymoves: &[bool], _oppmoves: &[bool]| -> bool {false}; // always steal
    let grudger = |_mymoves: &[bool], oppmoves: &[bool]| -> bool {
        if oppmoves.contains(&false) { // share, unless opponent steals, then always steal
            return false
        }
        true
    };
    let thankful = |_mymoves: &[bool], oppmoves: &[bool]| -> bool { // aka antigrudger
        if oppmoves.contains(&true) { // steal, unless opponent shares, then always share
            return true
        }
        false
    };
    let mimic = |_mymoves: &[bool], oppmoves: &[bool]| -> bool {
        match oppmoves.last() { // share, then whatever opponent did last turn
            None => true,
            Some(x) => *x,
        }
    };
    let antimimic = |_mymoves: &[bool], oppmoves: &[bool]| -> bool {
        match oppmoves.last() { // steal, then opposite whatever opponent did last turn
            None => false,
            Some(x) => !*x,
        }
    };
    let alternating = |mymoves: &[bool], _oppmoves: &[bool]| -> bool {
        match mymoves.last() { // share, steal, share, steal, ...
            None => true,
            Some(x) => !x,
        }
    };
    let random = |_mymoves: &[bool], _oppmoves: &[bool]| -> bool {
        match [true, false].choose(&mut rand::rng()) { // pick randomly
            None => unreachable!(),
            Some(x) => *x,
        }
    };
    let tester = |_mymoves: &[bool], oppmoves: &[bool]| -> bool {
        // split steal split slit, if opponent responds with steal on round 3, act like mimic, if opponent responds with share, act like greedy
        if oppmoves.is_empty() {return true}
        if oppmoves.len() == 1 {return false}
        if oppmoves.len() <= 3 {return true}
        if oppmoves[2] { // if opponent shared, swap to greedy
            false
        } else { // if opponent stole, swap to mimic
            match oppmoves.last() { // whatever opponent did last turn
                None => true,
                Some(x) => *x,
            }
        }
    };
    

    const STRATCOUNT: usize = 9;
    let strategies: [Stratfn; STRATCOUNT]
        = [generous, greedy, grudger, thankful, mimic, antimimic, alternating, random, tester];
    let names: [&str; STRATCOUNT] = ["generous", "greedy", "grudger", "thankful", "mimic", "antimimic", "alternating", "random", "tester"];
    let mut scores: [i32; STRATCOUNT] = [0; STRATCOUNT];
    assert!(strategies.len() == names.len());

    println!("Games");
    for a in 0..strategies.len() {
        for b in a..strategies.len() {
            let score = play(strategies[a], strategies[b], 100);
            scores[a] += score.0; scores[b] += score.1;
            println!("{} vs {}: ({}, {})", names[a], names[b], score.0, score.1);
        }
    }
    println!();
    println!("Scores");
    for i in 0..strategies.len() {
        println!("{}: {}", names[i], scores[i]);
    }
}

fn play(a: fn(&[bool], &[bool])->bool, b: fn(&[bool], &[bool])->bool, rounds: u32) -> (i32, i32) { // returns total points
    let mut moves_a = vec![]; // previous moves a has made
    let mut moves_b = vec![]; // previous moves b has made
    let mut sum_score = (0, 0); // total score
    for _round in 0..rounds { // play for n rounds
        let move_a = a(&moves_a, &moves_b);
        let move_b = b(&moves_b, &moves_a);
        let round_score = TABLE[if move_a {1} else {0} | if move_b {2} else {0}]; // will never exceed 3
        sum_score = (sum_score.0 + round_score.0, sum_score.1 + round_score.1);
        moves_a.push(move_a);
        moves_b.push(move_b);
    }
    sum_score
}