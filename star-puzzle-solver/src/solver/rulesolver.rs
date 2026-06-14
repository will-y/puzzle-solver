use crate::solver::rules::fillarray::FillArrayRule;
use crate::solver::rules::finishcolor::FinishColorRule;
use crate::solver::rules::fullarray::FullArrayRule;
use crate::solver::rules::linerule2star::LineRule2Star;
use crate::solver::rules::Rule;
use crate::solver::{Solver, SolverResult};
use star_puzzle::board::Board;

/// The goal of this solver is to use rules to solve it
/// like a human would. The file needs to include a bunch of different rules
/// that can be used for the rule solver (in a certain order)
pub struct RuleSolver {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleSolver {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    pub fn default() -> Self {
        Self::new(vec![
            Box::new(LineRule2Star {}),
            Box::new(FillArrayRule {}),
            Box::new(FullArrayRule {}),
            Box::new(FinishColorRule {})
        ])
    }

    /// Loops over all rules.
    ///
    /// Returns true if any rule provided any value.
    fn loop_over_rules(&self, board: &mut Board, applied_rule: &mut Vec<AppliedRule>) -> bool {
        let mut changed = false;
        self.rules.iter().for_each(|rule| {
            if rule.apply(board) {
                changed = true;
                applied_rule.push(AppliedRule {
                    name: rule.name(),
                    description: rule.short_description(),
                });
            }
        });

        changed
    }

    /// Guesses a star.
    ///
    /// This will either solve the board, or place a dot if it reaches a contradiction.
    fn guess_and_check(&self, board: &mut Board) {
        // Find star
        let star_point = self.find_star_to_guess(board);

        // Copy board
        let mut board_copy = board.clone();
        // Place on board
        match board_copy.place_star(star_point.0, star_point.1) {
            Ok(_) => {}
            Err(_) => {
                board.place_dot(star_point.0, star_point.1);
                return;
            }
        }
        // Check for contradictions
        if self.board_has_contradictions(&board_copy) {
            board.place_dot(star_point.0, star_point.1);
            return;
        }
        // Run rules until done or contradiction
        while self.loop_over_rules(&mut board_copy, &mut vec![]) {
            if self.board_has_contradictions(&board_copy) {
                board.place_dot(star_point.0, star_point.1);
                return;
            }
        }

        // TODO: Here it is either solved or retry recursively?
        // Keep a list of stars and dots? Then apply them all if solved?
        println!("If we get here things are going to break");
    }

    /// Finds the next star to guess.
    ///
    /// For now this just picks the next star in order.
    /// In the future I want this to be smarter (guessing places that would be helpful to have a star or dot).
    fn find_star_to_guess(&self, board: &Board) -> (usize, usize) {
        for x in 0..board.size {
            for y in 0..board.size {
                if board.is_empty(x, y) {
                    return (x, y);
                }
            }
        }

        panic!("No star to guess found, board should be solved at this point")
    }

    fn board_has_contradictions(&self, board: &Board) -> bool {
        // TODO: Need more things here?
        board
            .state
            .current_color_sections
            .iter()
            .enumerate()
            .any(|(i, section)| {
                section.positions.len() < board.max_star_count - *board.state.star_counts.get(&i).unwrap_or(&0)
            })
    }
}

impl Solver for RuleSolver {
    fn solve(&self, board: &mut Board) -> Box<dyn SolverResult> {
        // TODO: Filter rules to only ones that apply
        let mut applied_rules: Vec<AppliedRule> = vec![];

        while !board.is_solved() {
            let rule_changed = self.loop_over_rules(board, &mut applied_rules);
            println!("After Rule Iterations: ");
            board.print();

            if !rule_changed {
                self.guess_and_check(board);
                applied_rules.push(AppliedRule {
                    name: "Guess and Check".to_string(),
                    description: "Guess a star and check if it leads to a contradiction".to_string(),
                });
                println!("After guess and check");
                board.print();
            }
        }

        Box::new(RuleSolverResult { applied_rules})
    }
}

pub struct RuleSolverResult {
    applied_rules: Vec<AppliedRule>
}

impl SolverResult for RuleSolverResult {
    fn print_results(&self) {
        println!("Rule Solver Results:");
        println!("    Applied Rules:");
        self.applied_rules.iter().for_each(|rule| {
            println!("        {}: {}", rule.name, rule.description);
        });
    }

    fn format_results(&self) -> String {
        let mut result = String::new();
        result.push_str("Rule Solver Results:");
        result.push_str("\n    Applied Rules:");
        self.applied_rules.iter().for_each(|rule| {
            result.push_str("\n        ");
            result.push_str(&rule.name);
        });
        
        result
    }
}

/// A struct that keeps track of the rules that were applied.
#[derive(Debug)]
pub struct AppliedRule {
    name: String,
    description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_board() {
        let mut board = Board::from_string("0111222222\n0333332222\n0300332422\n0005552422\n0000000422\n0000222222\n0000067772\n0088862222\n6666669992\n6666666222", 2).unwrap();

        println!("Attempting to solve board");
        board.print();

        let solver = RuleSolver::default();

        let result = solver.solve(&mut board);

        result.print_results();

        assert!(board.is_solved());
    }
}
