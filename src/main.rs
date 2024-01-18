use std::time::Duration;

use crate::algorithms::the_algorithm::Algorithm;
use crate::common::constants::modules::{
    ALPHA_BETA, ANALYZE, SEARCH_EXTENSIONS, SKIP_BAD_MOVES, SQUARE_CONTROL_METRIC,
    TRANSPOSITION_TABLE,
};

use self::pitter::logic::{Competition, GameOutcome};

mod algorithms;
mod common;
mod pitter;

#[tokio::main]
async fn main() {
    let modules1 = SEARCH_EXTENSIONS | ALPHA_BETA | TRANSPOSITION_TABLE | SKIP_BAD_MOVES | ANALYZE;
    let modules2 = ALPHA_BETA | ANALYZE;
    let time_per_move1 = Duration::from_micros(12000);
    let time_per_move2 = Duration::from_micros(12000);

    let competition = Competition::new(
        Algorithm::new(modules1, time_per_move1),
        Algorithm::new(modules2, time_per_move2),
    );

    // competition.analyze_algorithm_choices(|(game_info, _), _| {
    //     game_info.outcome == GameOutcome::InconclusiveTooLong
    // });
    let results = competition.start_competition(100).await;
    dbg!(results);
}
