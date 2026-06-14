pub mod dfs;
pub mod bfs;
pub mod rulesolver;
pub mod rules;

use star_puzzle::board::Board;

// TODO: Generics here, one for result definitely, maybe one for parameters but that might just go into constructing the solver
// Result should always contain a time, so maybe a trait?
pub trait Solver {
    fn solve(&self, board: &mut Board) -> Box<dyn SolverResult>;
}

pub trait SolverResult {
    fn print_results(&self);
}