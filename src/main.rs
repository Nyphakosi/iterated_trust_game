//                (a,b)     a: steal  share      b:
// const TABLE: [(i32,i32); 4] = [(2,2), (2,8),  // steal
//                                (8,2), (5,5)]; // share
const TABLE: [(i32,i32); 4] = [(0,0), (-1,3), 
                               (3,-1), (2,2)];

#[derive(Debug)]
enum Strategy {
    Random,
    Generous,
    Greedy,
    Alternating,
    BiAlternating,
    QuinAlternating,
    PhaseAlternating,
    Mimic,
    ForgivingMimic,
    Antimimic,
    Grudger,
    ForgivingGrudger,
    Thankful,
    Tester,
}

impl Strategy {
    fn decide(&self, mymoves: &[bool], oppmoves: &[bool]) -> bool {
        use Strategy::*;
        match self {
            Random => rand::random_bool(0.5), // choose randomly
            Generous => true, // always share
            Greedy => false, // always steal
            Alternating => mymoves.len() % 2 < 1, // share steal share steal ...
            BiAlternating => mymoves.len() % 4 < 2, // share share, steal steal, ...
            QuinAlternating => mymoves.len() % 10 < 5, // share x5, steal x5, ...
            PhaseAlternating => !mymoves.len() % 2 < 1, // steal share steal share ...
            Mimic => *oppmoves.last().unwrap_or(&true), // share, then opponents last move
            ForgivingMimic => { // steal if opponent has stolen in the previous 2 rounds
                if oppmoves.len() < 2 {true} // share for first two turns
                else {!(!oppmoves[oppmoves.len()-2] && !oppmoves[oppmoves.len()-1])}
            }, 
            Antimimic => !oppmoves.last().unwrap_or(&true), // steal, then not opponents last move
            Grudger => !oppmoves.contains(&false), // generous, unless opponent steals, then greedy
            ForgivingGrudger => oppmoves.iter().filter(|b| !*b).count() < 2, // grudges if opponent steals twice
            Thankful => oppmoves.contains(&true), // greedy, unless opponent shares, then generous, aka antigrudger
            Tester => match oppmoves.len() { // share steal share share, if on round 3 opponent share, become greedy, else become mimic
                0 => true, 1 => false, 2 => true, 3 => true,
                _ => if oppmoves[2] { false } else { *oppmoves.last().unwrap_or(&true) }
            }
        }
    }
}

fn main() {
    // return true: share
    // return false: steal

    const STRATCOUNT: usize = 14;
    use Strategy::*;
    let strategies: [Strategy; STRATCOUNT] = [Random, Generous, Greedy, Alternating, BiAlternating, QuinAlternating, PhaseAlternating, Mimic, ForgivingMimic, Antimimic, Grudger, ForgivingGrudger, Thankful, Tester];
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