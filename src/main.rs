use std::fmt;

//                (a,b)     a: steal  share      b:
// const TABLE: [(i32,i32); 4] = [(2,2), (2,8),  // steal
//                                (8,2), (5,5)]; // share
const TABLE: [(i32,i32); 4] = [(0,0), (-1,3), 
                               (3,-1), (2,2)];

const ROUNDS: u32 = 100;
const MISPLAY_CHANCE: f64 = 0.05;

//#[derive(Debug)]
enum Strategy {
    Random(f64), // probability to share
    Generous,
    Greedy,
    Periodic(u32, u32), // period, seq
    Copycat,
    Copykitten,
    Anticat,
    Grudger,
    ForgivingGrudger,
    Thankful,
    CautiousThankful,
    Tester,
    Betrayer,
    Pavlov,
    Antipavlov,
    Thoughtful,
    CultLeader,
    Cultist(u8),
}

impl Strategy {
    fn decide(&self, mymoves: &[bool], oppmoves: &[bool]) -> bool {
        use Strategy::*;
        match self {
            Random(p) => rand::random_bool(*p), // choose randomly
            Generous => true, // always share
            Greedy => false, // always steal
            Periodic(p, b) => { // period, sequence of moves
                b & (1<<(p - (mymoves.len() as u32 % *p)-1)) != 0 // pick bit from binary num by logical AND with leftshifted 1
            },
            Copycat => *oppmoves.last().unwrap_or(&true), // share, then opponents last move
            Copykitten => { // steal if opponent has stolen in the previous 2 rounds
                if oppmoves.len() < 2 {true} // share for first two turns
                else {!(!oppmoves[oppmoves.len()-2] && !oppmoves[oppmoves.len()-1])}
            }, 
            Anticat => !oppmoves.last().unwrap_or(&true), // steal, then not opponents last move
            Grudger => !oppmoves.contains(&false), // generous, unless opponent steals, then greedy
            ForgivingGrudger => oppmoves.iter().filter(|b| !*b).count() < 2, // grudges if opponent steals twice
            Thankful => oppmoves.contains(&true), // greedy, unless opponent shares, then generous, aka antigrudger
            CautiousThankful => !oppmoves.iter().filter(|b| !*b).count() < 2, // thankful if opponent shares twice
            Tester => match oppmoves.len() { // share steal share share, if on round 3 opponent share, become greedy, else become copycat
                0 => true, 1 => false, 2 => true, 3 => true,
                _ => if oppmoves[2] { false } else { *oppmoves.last().unwrap_or(&true) }
            },
            Betrayer => {if mymoves.len() < ((ROUNDS*8/10) as usize) { // act like copycat, but in the last 20% of rounds, swap to greedy
                    *oppmoves.last().unwrap_or(&true)
                } else {false}
            }
            Pavlov => mymoves.last().unwrap_or(&true) == oppmoves.last().unwrap_or(&true),
            Antipavlov => mymoves.last().unwrap_or(&false) != oppmoves.last().unwrap_or(&false),
            Thoughtful if oppmoves.len() <= 5 => [true, true, false, true, false, true][oppmoves.len()],
            Thoughtful => { // probabilistic model, makes decisions depending on payout chance
                const PAYRATE: f64 = 0.5;
                const Z: f64 = 1.5;
                let n = (oppmoves.len() - 1) as f64;
                let r = (PAYRATE + 1.0) * 0.5;
                let mut total = [0, 0];
                let mut coincident = [0, 0];
                for (m, t) in mymoves[..mymoves.len() - 1].iter().zip(oppmoves[1..].iter()) {
                    let i = if *m { 1 } else { 0 };
                    total[i] += 1;
                    if m == t { coincident[i] += 1; }
                }
                let [to0, to1] = total;
                let [co0, co1] = coincident;
                let mu0 = co0 as f64 / to0 as f64;
                let var0 = mu0 * (1.0 - mu0) / to0 as f64;
                let mu1 = co1 as f64 / to1 as f64;
                let var1 = mu1 * (1.0 - mu1) / to1 as f64;
                let c = mu0 + mu1 - 1.0; // reciprocity
                let sd_c = (var0 + var1).sqrt();
                let eventide_var = (mu0 * (1.0 - mu0) + mu1 * (1.0 - mu1)) * 2.0 / n;
                let eventide_sd = eventide_var.sqrt();
                // eprintln!("{c} {sd_c}");
                if c - sd_c * Z > 1.0 - r {
                    true
                } else if c + sd_c * Z >= 1.0 - r {
                    if sd_c / eventide_sd >= 1.25 {
                        // eprintln!("rebalance [{}]", oppmoves.len());
                        to0 >= to1
                    } else {
                        *oppmoves.last().unwrap_or(&true)
                    }
                } else {
                    false
                }
            }
            CultLeader if oppmoves.len() <= 5 => [false, true, false, true, true, false][oppmoves.len()],
            CultLeader => { // if paired against follower, become greedy, else become copycat
                if oppmoves[0..6] == [true, false, true, false, false, true] { 
                    false
                } else {
                    *oppmoves.last().unwrap_or(&true)
                }
            },
            Cultist(x) if oppmoves.len() <= 5 => [true, false, true, false, false, true][oppmoves.len()],
            Cultist(_x) => { // if paired against leader, become generous, else become greedy
                oppmoves[0..6] == [false, true, false, true, true, false]
            },
        }
    }
}

impl std::fmt::Debug for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Strategy::*;
        match self {
            Random(p) => write!(f, "Random ({p})"), 
            Generous => write!(f, "Generous"), 
            Greedy => write!(f, "Greedy"), 
            Periodic(p, b) => {let p = *p as usize; write!(f, "Periodic ({b:0p$b})")}, 
            Copycat => write!(f, "Copycat"), 
            Copykitten => write!(f, "Copykitten"), 
            Anticat => write!(f, "Anticat"), 
            Grudger => write!(f, "Grudger"), 
            ForgivingGrudger => write!(f, "Forgiving Grudger"), 
            Thankful => write!(f, "Thankful"), 
            CautiousThankful => write!(f, "Cautious Thankful"), 
            Tester => write!(f, "Tester"), 
            Betrayer => write!(f, "Betrayer"), 
            Pavlov => write!(f, "Pavlov"), 
            Antipavlov => write!(f, "Antipavlov"), 
            Thoughtful => write!(f, "Thoughtful"), 
            CultLeader => write!(f, "Cult Leader"), 
            Cultist(n) => write!(f, "Cultist {n}"), 
        }
    }
}

fn main() {
    // return true: share
    // return false: steal

    use Strategy::*;
    let strategies = 
        [Random(0.25), Random(0.50), Random(0.75), Generous, Greedy, 
         Periodic(2, 0b10), Periodic(2, 0b01), Periodic(4, 0b1100), Periodic(10, 0b1111100000), 
         Copycat, Copykitten, Anticat, 
         Grudger, ForgivingGrudger, Thankful, CautiousThankful, 
         Tester, Betrayer, Pavlov, Antipavlov, Thoughtful, 
         CultLeader, Cultist(1), Cultist(2), Cultist(3), Cultist(4), Cultist(5), 
         ];
    let stratcount = strategies.len();
    let mut scores = vec![0; stratcount];

    println!("Games");
    for a in 0..strategies.len() {
        for b in a..strategies.len() {
            let score = play(&strategies[a], &strategies[b], ROUNDS);
            scores[a] += score.0; scores[b] += score.1;
            println!("({:>3}, {:>3}) | {:?} vs {:?}", score.0, score.1, strategies[a], strategies[b]);
        }
    }

    let mut scoreboard: Vec<(&Strategy, i32)> = vec![];
    for i in 0..stratcount {
        scoreboard.push((&strategies[i], scores[i]))
    }
    scoreboard.sort_by_key(|k| k.1);
    scoreboard.reverse();
    println!();
    println!("Scores");
    for i in scoreboard {
        println!("{:>5} | {:?}", i.1, i.0);
    }
}

fn play(a: &Strategy, b: &Strategy, rounds: u32) -> (i32, i32) { // returns total points
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

fn _play_debug(a: &Strategy, b: &Strategy, rounds: u32) -> (i32, i32) { // returns total points
    let mut moves_a = vec![]; // previous moves a has made
    let mut moves_b = vec![]; // previous moves b has made
    let mut sum_score = (0, 0); // total score
    for _round in 0..rounds { // play for n rounds
        let move_a = a.decide(&moves_a, &moves_b);
        let move_b = b.decide(&moves_b, &moves_a);
        println!("{move_a:>5}, {move_b:>5}");
        let round_score = TABLE[if move_a {1} else {0} | if move_b {2} else {0}]; // will never exceed 3
        sum_score = (sum_score.0 + round_score.0, sum_score.1 + round_score.1);
        moves_a.push(move_a);
        moves_b.push(move_b);
    }
    sum_score
}