use star_puzzle::board::Board;
use crate::solver::{Solver};

pub mod solver;

fn main() {
    println!("Starting Solver ...");
    let board = Board::from_string("00011\n21113\n21433\n44433\n44444", 1).unwrap();
    let board_2 = Board::from_string("0111222222\n0333332222\n0300332422\n0005552422\n0000000422\n0000222222\n0000067772\n0088862222\n6666669992\n6666666222", 2).unwrap();

    let solvers: Vec<Box<dyn Solver>> = vec![
        Box::new(solver::dfs::DfsSolver::new(false)),
        Box::new(solver::bfs::BfsSolver::new(false))
    ];

    run_solvers(board, &solvers);
    run_solvers(board_2, &solvers);
}

fn run_solvers(board: Board, solvers: &Vec<Box<dyn Solver>>) {
    println!("Board to Solve:");
    board.print();

    // TODO: Figure out moving / references again so I don't need extra clones
    solvers.iter().for_each(|solver| {
        let result = solver.solve(board.clone());
        result.print_results();
    });
}
