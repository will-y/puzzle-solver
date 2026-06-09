use star_puzzle::board::Board;
use crate::solver::Solver;

pub mod solver;

fn main() {
    println!("Starting Solver ...");
    let mut board = Board::from_string("00011\n21113\n21433\n44433\n44444", 1).unwrap();

    println!("Board to Solve:");
    board.print();

    solver::dfs::DfsSolver::new().solve(board);
}
