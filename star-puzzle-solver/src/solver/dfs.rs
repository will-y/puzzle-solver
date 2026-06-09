use star_puzzle::board::Board;
use crate::solver::Solver;

#[derive(PartialEq, Debug)]
pub struct DfsSolver {

}

impl Solver for DfsSolver {
    fn solve(&self, board: Board) {
        self.dfs_solve(board);
    }
}

impl DfsSolver {
    pub fn new() -> DfsSolver {
        DfsSolver {}
    }

    fn dfs_solve(&self, board: Board) {
        let mut stack: Vec<Board> = vec!();
        stack.push(board);

        while !stack.is_empty() {
            let current_board = stack.pop().unwrap();
            println!("--------");
            current_board.print();
            if current_board.is_solved() {
                println!("Solved!");
                return;
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
    }
}