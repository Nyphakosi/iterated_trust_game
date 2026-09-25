#![allow(non_snake_case)]
use crate::Strategy;

mod simplistic;
mod Nyphakosi;
mod Raindrops;
mod relyks;

pub(super) fn retrieve_strategies() -> Vec<Box<dyn Strategy>> {
    let mut strats = simplistic::retrieve_strategies();
    strats.extend(Nyphakosi::retrieve_strategies());
    strats.extend(Raindrops::retrieve_strategies());
    strats.extend(relyks::retrieve_strategies());
    strats
}

