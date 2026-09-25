#![allow(dead_code)]
use std::fmt;
use crate::{MISPLAY_CHANCE, ROUNDS, Strategy, TABLE};

#[derive(Clone)]
pub struct Businessman{
    z: f64, // required confidence sigmas
    b: f64, // detect imbalanced samples by thresholding real stdev / ideal stdev
    decay: f64, // discount old impressions
    opening: Box<[bool]>,

    i: usize, // opening index
    n: [f64; 2], // total
    m: [f64; 2], // coincidental
    my_last: bool, // coincidence check happens across turns, so this is necessary
}
impl fmt::Display for Businessman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Businessman({},{},{}){{{:?}}}", self.z, self.b, self.decay, self.opening)
    }
}
impl Businessman {
    pub fn new(
        z: f64, b: f64, decay: f64,
        opening: impl 'static + Into<Box<[bool]>>,
    ) -> Self {
        assert!((0.0..=1.0).contains(&decay));
        let opening = opening.into();
        assert!(opening.len() >= 2);
        Self { z, b, decay, opening, i: 0, n: [0., 0.], m: [0., 0.], my_last: false }
    }
    fn approximate_linear(payout: &[f64; 4]) -> (f64, f64, f64) {
        let &[d, c, b, a] = payout;
        // we assume the payout is linear, in the form of f(u,v) = w + su + tv
        // we estimate w, s, t
        //     w + s + t = a
        //     w + s     = b
        //     w     + t = c
        //     w         = d
        // we recognize that this is an overdetermined set of linear equations in the form
        //   A x == b
        // and hence we proceed by solving
        //   A⊤ A x = A⊤ b
        let t = 0.5 * (a - b + c - d);
        let s = 0.5 * (a + b - c - d);
        let w = 0.25 * (-a + b + c + 3.0*d);
        (w, s, t)
    }
}
struct GameSettings {
    pub n_rounds: usize,
    pub payout: [f64; 4],  // index msb=mine, lsb=theirs
    pub miss_rate: f64,
}
const G: GameSettings = GameSettings {
        n_rounds: ROUNDS as usize,
        payout: [TABLE[0].1 as f64, TABLE[1].1 as f64, TABLE[2].1 as f64, TABLE[3].1 as f64],
        miss_rate: MISPLAY_CHANCE,
};
impl Strategy for Businessman { // not yet implemented, acts like generous
    fn decide(&mut self, memory: &[bool], history: &[bool]) -> bool {
        let update = {
            if history.is_empty() {Option::None}
            else {Some((memory[memory.len()-1], history[history.len()-1]))}
        };
        match update {
            None => {
                assert_eq!(self.i, 0);
                self.i += 1;
                self.opening[0]
            }
            Some((mine, theirs)) => {
                assert_ne!(self.i, 0);
                self.n[0] *= self.decay; self.n[1] *= self.decay;
                self.m[0] *= self.decay; self.m[1] *= self.decay;
                if self.i >= 2 {
                    let j = if self.my_last {1} else {0};
                    self.n[j] += 1.0;
                    if self.my_last == theirs { self.m[j] += 1.0; }
                }
                self.my_last = mine;
                let old_i = self.i;
                self.i += 1;
                if old_i < self.opening.len() {
                    return self.opening[old_i]
                }
                let (μ0, var0) = bernouli_mean_var(self.n[0], self.m[0]);
                let (μ1, var1) = bernouli_mean_var(self.n[1], self.m[1]);
                let r = μ0 + μ1 - 1.0;
                let sd_r = (var0 + var1).sqrt();
                let sd_r_ideal =
                    ((μ0 * (1.0 - μ0) + μ1 * (1.0 - μ1)) * 2.0 / (self.n[0] + self.n[1])).sqrt();
                let (_w, s, t) = Self::approximate_linear(&G.payout);
                let (r_lo, r_hi) = (r - sd_r * self.z, r + sd_r * self.z);
                if s + t * r_lo > 0.0 {
                    true
                } else if s + t * r_hi >= 0.0 {
                    if sd_r / sd_r_ideal > self.b {
                        self.n[0] >= self.n[1]
                    } else {
                        theirs
                    }
                } else {
                    false
                }
            }
        }
    }
}
fn bernouli_mean_var(total: f64, success: f64) -> (f64, f64) {
    let μ = success / total;
    let var = μ * (1.0 - μ) / total;
    (μ, var)
}

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        Box::new(Businessman::new(2.1, 1.45, 0.98, [true, true, false, true, true, false, true, true]))
    ]
}
