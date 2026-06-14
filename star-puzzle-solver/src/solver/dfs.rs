use std::collections::HashSet;
use std::time::{Duration, Instant};
use star_puzzle::board::{Board};
use crate::solver::{Solver, SolverResult};

#[derive(PartialEq, Debug)]
pub struct DfsResult {
    time: Duration,
    states_explored: usize,
    found_solution: bool
}

impl SolverResult for DfsResult {
    fn print_results(&self) {
        println!("DFS Solver Results");
        println!("    Time: {:?}", self.time);
        println!("    States Explored: {}", self.states_explored);
        println!("    Found Solution: {}", self.found_solution);
    }

    fn format_results(&self) -> String {
        String::from("TODO: BFS Solver Results:")
    }
}

#[derive(PartialEq, Debug)]
pub struct DfsSolver {
    print: bool
}

impl Solver for DfsSolver {
    fn solve(&self, board: &mut Board) -> Box<dyn SolverResult> {
        let now = Instant::now();
        let dsf_results = self.dfs_solve(board);

        Box::new(DfsResult {
            time: now.elapsed(),
            states_explored: dsf_results.0,
            found_solution: dsf_results.1
        })
    }
}

impl DfsSolver {
    pub fn new(print: bool) -> DfsSolver {
        DfsSolver { print }
    }

    fn dfs_solve(&self, board: &mut Board) -> (usize, bool) {
        let mut visited = HashSet::new();
        let mut stack: Vec<Board> = vec!();
        stack.push(board.clone());

        while !stack.is_empty() {
            let current_board = stack.pop().unwrap();
            if !visited.insert(current_board.state.clone()) {
                continue;
            }

            if self.print {
                println!("--------");
                current_board.print();
            }

            if current_board.is_solved() {
                return (visited.len(), true)
            }

            // Go through board until an empty spot is found
            for x in 0..current_board.size {
                for y in 0..current_board.size {
                    if current_board.is_empty(x, y) {
                        let mut new_board = current_board.clone();
                        if new_board.place_star(x, y).is_ok() {
                            stack.push(new_board);
                        }
                    }
                }
            }
        }

        (visited.len(), false)
    }
}