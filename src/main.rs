//                (a,b)     a: steal  share      b:
// const TABLE: [(i32,i32); 4] = [(2,2), (2,8),  // steal
//                                (8,2), (5,5)]; // share
const TABLE: [(i32,i32); 4] = [(0,0), (-1,3), 
                               (3,-1), (2,2)];

#[derive(Debug)]
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
            Generous => true, // always split
            Greedy => false, // always steal
            Grudger => !oppmoves.contains(&false), // generous, unless opponent steals, then greedy
            Thankful => oppmoves.contains(&true), // greedy, unless opponent shares, then generous
            Mimic => *oppmoves.last().unwrap_or(&true), // split, then opponents last move
            Antimimic => !oppmoves.last().unwrap_or(&true), // steal, then not opponents last move
            Alternating => mymoves.len().is_multiple_of(2), // atlernate split steal
            Random => rand::random_bool(0.5),
            Tester => match oppmoves.len() { // split steal split split, if on round 3 opponent split, become greedy, else become mimic
                0 => true, 1 => false, 2 => true, 3 => true,
                _ => if oppmoves[2] { false } else { *oppmoves.last().unwrap_or(&true) }
            }
        }
    }
}

fn main() {
    // return true: share
    // return false: steal

    const STRATCOUNT: usize = 9;
    use Strategy::*;
    let strategies: [Strategy; STRATCOUNT] = [Generous, Greedy, Grudger, Thankful, Mimic, Antimimic, Alternating, Random, Tester];
    let mut scores: [i32; STRATCOUNT] = [0; STRATCOUNT];

    println!("Games");
    for a in 0..strategies.len() {
        for b in a..strategies.len() {
            let score = play(&strategies[a], &strategies[b], 10);
            scores[a] += score.0; scores[b] += score.1;
            println!("{:?} vs {:?}: ({}, {})", strategies[a], strategies[b], score.0, score.1);
        }
    }
    println!();
    println!("Scores");
    for i in 0..strategies.len() {
        println!("{:?}: {}", strategies[i], scores[i]);
    }
}

fn play(a: &Strategy, b: &Strategy, rounds: u32) -> (i32, i32) { // returns total points
    let mut moves_a = vec![]; // previous moves a has made
    let mut moves_b = vec![]; // previous moves b has made
    let mut sum_score = (0, 0); // total score
    for _round in 0..rounds { // play for n rounds
        let move_a = a.decide(&moves_a, &moves_b);
        let move_b = b.decide(&moves_b, &moves_a);
        let round_score = TABLE[if move_a {1} else {0} | if move_b {2} else {0}]; // will never exceed 3
        sum_score = (sum_score.0 + round_score.0, sum_score.1 + round_score.1);
        moves_a.push(move_a);
        moves_b.push(move_b);
    }
    sum_score
}