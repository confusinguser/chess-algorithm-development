use std::collections::HashMap;

use chess::Board;
use tokio::time::Instant;

use crate::algorithms::utils::Evaluation;
use crate::common::utils::Stats;

#[derive(Debug, Copy, Clone)]
pub struct TranspositionEntry {
    pub depth: u32,
    pub(crate) evaluation: Evaluation,
}

impl TranspositionEntry {
    pub(crate) fn new(depth: u32, evaluation: Evaluation) -> Self {
        TranspositionEntry { depth, evaluation }
    }
}

pub(crate) fn insert_in_transposition_table(
    transposition_table: &mut HashMap<Board, TranspositionEntry>,
    board: &Board,
    depth: u32,
    stats: &mut Stats,
    evaluation: Evaluation,
) {
    let start = Instant::now();
    transposition_table.insert(*board, TranspositionEntry::new(depth, evaluation));
    stats.time_for_transposition_access += Instant::now() - start;
    stats.transposition_table_entries += 1
}

pub(crate) fn get_transposition_entry(
    transposition_table: &HashMap<Board, TranspositionEntry>,
    stats: &mut Stats,
    board: &Board,
) -> Option<TranspositionEntry> {
    let start = Instant::now();

    let transposition_entry = transposition_table.get(board).copied();

    let time_for_transposition_access = Instant::now() - start;
    stats.time_for_transposition_access += time_for_transposition_access;

    transposition_entry
}
