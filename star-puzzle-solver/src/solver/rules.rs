pub mod fullarray;
pub mod fillarray;
pub mod finishcolor;
pub mod shape;
pub mod colorarray;

use star_puzzle::board::Board;

pub trait Rule {
    /// Applies the given rule to the board.
    /// It returns true if the rule made some change, false otherwise.
    /// A return of true does not mean that it will not find more things to do if ran again on the same board.
    /// Returns Error if a star cannot be placed
    fn apply(&self, board: &mut Board) -> Result<bool, String>;

    /// Returns true if this rule can be applied to the given board.
    /// For example, there can be rules specific to 1-star puzzles.
    ///
    /// By default, rules can apply to all boards
    fn can_apply(&self, _board: &Board) -> bool {
        true
    }

    fn name(&self) -> String;

    fn short_description(&self) -> String;
}