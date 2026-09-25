use crate::Strategy;

pub mod simplistic;
#[allow(non_snake_case)]
pub mod Nyphakosi;
#[allow(non_snake_case)]
pub mod Raindrops;

pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    let mut strats = simplistic::retrieve_strategies();
    strats.extend(&Nyphakosi::retrieve_strategies());
    strats.extend(&Raindrops::retrieve_strategies());
    strats
}

