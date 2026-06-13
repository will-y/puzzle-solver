use star_puzzle::board::Board;
use crate::solver::{Solver, SolverResult};
use crate::solver::rules::linerule2star::LineRule2Star;
use crate::solver::rules::Rule;

/// The goal of this solver is to use rules to solve it
/// like a human would. The file needs to include a bunch of different rules
/// that can be used for the rule solver (in a certain order)
pub struct RuleSolver {
    rules: Vec<Box<dyn Rule>>
}

impl Solver for RuleSolver {
    fn solve(&self, board: Board) -> Box<dyn SolverResult> {
        // TODO: Filter rules to only ones that applu
        let mut solving_board = board.clone();
        while !solving_board.is_solved() {
            let rule_changed = self.loop_over_rules(&mut solving_board);

            if !rule_changed {

            }
        }

        Box::new(RuleSolverResult { })
    }
}

impl RuleSolver {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    pub fn default() -> Self {
        Self::new(vec![Box::new(LineRule2Star { })])
    }

    /// Loops over all rules.
    ///
    /// Returns true if any rule provided any value.
    fn loop_over_rules(&self, board: &mut Board) -> bool {
        let mut changed = false;
        self.rules.iter().for_each(|rule| {
            if rule.apply(board) {
                changed = true;
            }
        });

        changed
    }
}

pub struct RuleSolverResult {

}

impl SolverResult for RuleSolverResult {
    fn print_results(&self) {}
}