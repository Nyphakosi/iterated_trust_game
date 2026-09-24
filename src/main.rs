use std::fmt;

//                (a,b)     a: steal  share      b:
// const TABLE: [(i32,i32); 4] = [(2,2), (2,8),  // steal
//                                (8,2), (5,5)]; // share
const TABLE: [(i32,i32); 4] = [(0,0), (-1,3), 
                               (3,-1), (2,2)];

const ROUNDS: u32 = 1000;
const COPIES: u32 = 1;
const MISPLAY_CHANCE: f64 = 0.05;

#[derive(Clone)]
enum Strategy {
    Random(f64), // probability to share
    Generous,
    Greedy,
    Periodic(u32, u32), // period, seq
    Copycat,
    DelayedCopycat(usize), // how many turns delay
    Copykitten,
    Anticat,
    Grudger,
    ForgivingGrudger,
    Thankful,
    CautiousThankful,
    Tester,
    Betrayer(f64), // proportion of rounds to share
    Pavlov,
    Antipavlov,
    Businessman,
    CultLeader,
    Cultist(u8),
    Peasant(u8),
    King,
    Index(u8), // one of 32 with single-round memory
    Chancecat(f64, f64), // chance to act like copycat, chance to share
    Simpleton,
}

impl Strategy {
    fn decide(&self, mymoves: &[bool], oppmoves: &[bool]) -> bool {
        const CULT_LEADER_KEY: [bool; 8] = [true, false, false, true, true, false, true, false];
        const CULTIST_KEY: [bool; 8] = [false, true, true, false, false, true, false, true];
        const PEASANT_KEY: [bool; 8] = [false, true, false, true, true, true, false, false];
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
            Tester if mymoves.len() < 4 => [true, false, true, true][mymoves.len()],
            Tester => { // share steal share share, if on round 3 opponent share, become greedy, else become copycat
                if oppmoves[2] { false } else { *oppmoves.last().unwrap_or(&true) }
            },
            Betrayer(p) => {if mymoves.len() < ((ROUNDS as f64 * p) as usize) { // act like copycat for the first p% of rounds, swap to greedy
                    *oppmoves.last().unwrap_or(&true)
                } else {false}
            }
            Pavlov => mymoves.last().unwrap_or(&true) == oppmoves.last().unwrap_or(&true),
            Antipavlov => mymoves.last().unwrap_or(&false) != oppmoves.last().unwrap_or(&false),
            Businessman if oppmoves.len() < 8 => [true, true, false, true, true, false, true, true][oppmoves.len()],
            Businessman => { // probabilistic model, makes decisions depending on payout chance
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
            CultLeader if oppmoves.len() < CULT_LEADER_KEY.len() => CULT_LEADER_KEY[oppmoves.len()],
            CultLeader => { // if paired against Cultist, become greedy, else become copycat
                if {
                    let mut sum_ones = 0;
                    for i in 0..CULTIST_KEY.len() {
                        if oppmoves[i] ^ CULTIST_KEY[i] {sum_ones += 1}
                    }
                    sum_ones
                } < 2 { // if key matches all but one flipped bit, still allow it
                    false
                } else {
                    *oppmoves.last().unwrap_or(&true)
                }
            },
            Cultist(x) if oppmoves.len() < CULTIST_KEY.len() => CULTIST_KEY[oppmoves.len()],
            Cultist(_x) => { // if paired against CultLeader, become generous, else become greedy
                ({
                    let mut sum_ones = 0;
                    for i in 0..CULT_LEADER_KEY.len() {
                        if oppmoves[i] ^ CULT_LEADER_KEY[i] {sum_ones += 1}
                    }
                    sum_ones
                } < 2) // if key matches all but one flipped bit, still allow it
            },
            Peasant(_x) if mymoves.len() < PEASANT_KEY.len() => PEASANT_KEY[mymoves.len()],
            Peasant(_x) => { // if paired against Peasant, become generous, else become greedy
                ({
                    let mut sum_ones = 0;
                    for i in 0..PEASANT_KEY.len() {
                        if oppmoves[i] ^ PEASANT_KEY[i] {sum_ones += 1}
                    }
                    sum_ones
                } < 2) // if key matches all but one flipped bit, still allow it
            },
            King if mymoves.len() < PEASANT_KEY.len() => PEASANT_KEY[mymoves.len()],
            King => false,
            Index(n) => { // 1-move context strategy based on an index
                if mymoves.is_empty() {return n & 0b10000 != 0}
                match (mymoves.last().unwrap_or(&true), oppmoves.last().unwrap_or(&true)) {
                    (false, false) => n & 0b00001 != 0,
                    (false,  true) => n & 0b00010 != 0,
                    ( true, false) => n & 0b00100 != 0,
                    ( true,  true) => n & 0b01000 != 0,
                }
            },
            Chancecat(pc, ps) => {
                if rand::random_bool(*pc) {
                    *oppmoves.last().unwrap_or(&true)
                } else {
                    rand::random_bool(*ps)
                }
            },
            Simpleton => (!*oppmoves.last().unwrap_or(&true)) ^ (*mymoves.last().unwrap_or(&true)),
            DelayedCopycat(d) if mymoves.len() < *d => true,
            DelayedCopycat(d) => oppmoves[oppmoves.len()-*d],
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
            DelayedCopycat(d) => write!(f, "Delayed Copycat ({d})"), 
            Copykitten => write!(f, "Copykitten"), 
            Anticat => write!(f, "Anticat"), 
            Grudger => write!(f, "Grudger"), 
            ForgivingGrudger => write!(f, "Forgiving Grudger"), 
            Thankful => write!(f, "Thankful"), 
            CautiousThankful => write!(f, "Cautious Thankful"), 
            Tester => write!(f, "Tester"), 
            Betrayer(p) => write!(f, "Betrayer ({p})"), 
            Pavlov => write!(f, "Pavlov"), 
            Antipavlov => write!(f, "Antipavlov"), 
            Businessman => write!(f, "Businessman"), 
            CultLeader => write!(f, "CultLeader"), 
            Cultist(n) => write!(f, "Cultist {n}"), 
            Peasant(n) => write!(f, "Peasant {n}"), 
            King => write!(f, "King"), 
            Index(n) => write!(f, "Index {n}"), 
            Chancecat(pc, ps) => write!(f, "Chancecat ({pc}, {ps})"), 
            Simpleton => write!(f, "Simpleton"), 
        }
    }
}

fn main() {
    // return true: share
    // return false: steal

    use Strategy::*;
    let mut strategytypes = 
        vec![
         Random(0.25), Random(0.50), Random(0.75), 
         Generous, Greedy, 
         //Periodic(2, 0b10), Periodic(2, 0b01), Periodic(4, 0b1100), Periodic(10, 0b1111100000), 
         Copycat, Copykitten, Anticat, Simpleton, 
         DelayedCopycat(2), DelayedCopycat(4), DelayedCopycat(8), 
         Grudger, ForgivingGrudger, Thankful, CautiousThankful, 
         Tester, 
         Betrayer(0.25), Betrayer(0.50), Betrayer(0.75),
         Pavlov, Antipavlov, Businessman, 
         Chancecat(0.25, 0.5), Chancecat(0.5, 0.5), Chancecat(0.75, 0.5), Chancecat(0.90, 0.0),
         CultLeader, King,
         ];
    for i in 0..8 {
        strategytypes.push(Cultist(i as u8))
    }
    for i in 0..4 {
        strategytypes.push(Peasant(i as u8))
    }
    for i in 1..31 { // avoid adding greedy/generous again
        strategytypes.push(Index(i as u8))
    }
    for l in 2..=4 {
        for s in 1..((1<<l)-1) { // avoid adding greedy/generous again
            strategytypes.push(Periodic(l, s))
        }
    }
    let strategies = {
        let mut temp = vec![];
        for strat in strategytypes {
            for _ in 0..COPIES {temp.push(strat.clone())}
        }
        temp
    };
    let stratcount = strategies.len();
    let mut scores = vec![0; stratcount];

    println!("Playing Games...");
    for a in 0..strategies.len() {
        for b in a..strategies.len() {
            let score = play(&strategies[a], &strategies[b], ROUNDS);
            scores[a] += score.0; scores[b] += score.1;
            //println!("({:>4}, {:>4}) | {:?} vs {:?}", score.0, score.1, strategies[a], strategies[b]);
        }
    }

    let mut scoreboard: Vec<(&Strategy, i32)> = vec![];
    for i in 0..stratcount {
        scoreboard.push((&strategies[i], scores[i]))
    }
    scoreboard.sort_by_key(|k| k.1);
    scoreboard.reverse();
    println!();
    println!("Scores for {} rounds at {}% misplay chance", ROUNDS, MISPLAY_CHANCE*100.0);
    for i in scoreboard.iter().enumerate() {
        println!("{:>3}: {:>6} | {:?}", i.0+1, i.1.1, i.1.0);
    }

    // let mut scores = [0; 32];
    // println!("Playing Games");
    // for a in 0..32 {
    //     for b in a..32 {
    //         let score = play(&Index(a as u8), &Index(b as u8), ROUNDS);
    //         scores[a] += score.0; scores[b] += score.1;
    //         //println!("({:>3}, {:>3}) | {:?} vs {:?}", score.0, score.1, strategies[a], strategies[b]);
    //     }
    // }
    // let mut scoreboard: Vec<(i32, i32)> = vec![];
    // for i in 0..32 {
    //     scoreboard.push((i, scores[i as usize]))
    // }
    // scoreboard.sort_by_key(|k| k.1);
    // scoreboard.reverse();
    // println!();
    // println!("Scores");
    // for i in scoreboard {
    //     println!("{:>5} | Index {:?}", i.1, i.0);
    // }
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