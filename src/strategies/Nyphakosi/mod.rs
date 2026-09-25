use crate::Strategy;

mod kingmaker;
mod index;
mod periodic;
mod betrayer;

pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    let mut strats = kingmaker::retrieve_strategies();
    strats.extend(&index::retrieve_strategies());
    strats.extend(&periodic::retrieve_strategies());
    strats.extend(&betrayer::retrieve_strategies());
    strats
}