use crate::Strategy;

mod pavlov;
mod businessman;
mod chancecat;

pub(super) fn retrieve_strategies() -> Vec<&'static dyn Strategy> {
    let mut strats = pavlov::retrieve_strategies();
    strats.extend(&businessman::retrieve_strategies());
    strats.extend(&chancecat::retrieve_strategies());
    strats
}