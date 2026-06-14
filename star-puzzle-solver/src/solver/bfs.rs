use std::collections::VecDeque;
use std::time::{Duration, Instant};
use star_puzzle::board::Board;
use crate::solver::{Solver, SolverResult};

#[derive(PartialEq, Debug)]
pub struct BfsResult {
    time: Duration,
    states_explored: usize,
    found_solution: bool
}

impl SolverResult for BfsResult {
    fn print_results(&self) {
        println!("BFS Solver Results:");
        println!("    Time: {:?}", self.time);
        println!("    States Explored: {}", self.states_explored);
        println!("    Found Solution: {}", self.found_solution);
    }

    fn format_results(&self) -> String {
        String::from("TODO: BFS Solver Results:")
    }
}

#[derive(PartialEq, Debug)]
pub struct BfsSolver {
    print: bool
}

impl Solver for BfsSolver {
    fn solve(&self, board: &mut Board) -> Box<dyn SolverResult> {
        let now = Instant::now();
        let bsf_results = self.bfs_solve(board);

        Box::new(BfsResult {
            time: now.elapsed(),
            states_explored: bsf_results.0,
            found_solution: bsf_results.1
        })
    }
}

impl BfsSolver {
    pub fn new(print: bool) -> BfsSolver {
        BfsSolver { print }
    }

    fn bfs_solve(&self, board: &mut Board) -> (usize, bool) {
        let mut states_explored = 0;
        let mut queue: VecDeque<Board> = VecDeque::new();
        queue.push_back(board.clone());

        while !queue.is_empty() {
            let current_board = queue.pop_front().unwrap();
            states_explored += 1;

            if self.print {
                println!("--------");
                current_board.print();
            }

            if current_board.is_solved() {
                return (states_explored, true)
            }

            // Go through board until an empty spot is found
            for x in 0..current_board.size {
                for y in 0..current_board.size {
                    if current_board.is_empty(x, y) {
                        let mut new_board = current_board.clone();
                        if new_board.place_star(x, y).is_ok() {
                            queue.push_back(new_board);
                        }
                    }
                }
            }
        }

        (states_explored, false)
    }
}