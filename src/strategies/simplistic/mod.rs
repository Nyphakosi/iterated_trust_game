use crate::Strategy;

pub mod greedy_generous;
pub mod grudger_thankful;
pub mod copycat;
pub mod random;

pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    let mut strats = greedy_generous::retrieve_strategies();
    strats.extend(&copycat::retrieve_strategies());
    strats.extend(&grudger_thankful::retrieve_strategies());
    strats.extend(&random::retrieve_strategies());
    strats
}
