use star_puzzle::board::Board;
use crate::solver::rules::Rule;

/// This rule is used to fill up a given row or column if it has the max number of stars.
pub struct FullArrayRule {}

impl FullArrayRule {
    fn fill_rows(&self, board: &mut Board) -> bool {
        let mut changed = false;

        for i in 0..board.size {
            if board.state.row_counts[i] == board.max_star_count {
                for j in 0..board.size {
                    changed = board.place_dot(j, i) || changed;
                }
            }
        }

        changed
    }

    fn fill_columns(&self, board: &mut Board) -> bool {
        let mut changed = false;

        for i in 0..board.size {
            if board.state.col_counts[i] == board.max_star_count {
                for j in 0..board.size {
                    changed = board.place_dot(i, j) || changed;
                }
            }
        }

        changed
    }
}

impl Rule for FullArrayRule {
    fn apply(&self, board: &mut Board) -> Result<bool, String> {
        let rows_filled = self.fill_rows(board);
        let columns_filled = self.fill_columns(board);

        Ok(rows_filled || columns_filled)
    }

    fn name(&self) -> String {
        String::from("Fill row / column with dots")
    }

    fn short_description(&self) -> String {
        String::from("Fills a row or column with dots if it has the max number of stars in it")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_apply_dots() {
        let mut board = Board::from_string("00011\n21113\n21433\n44433\n44444", 1).unwrap();

        board.place_star(0, 1).unwrap();

        let rule = FullArrayRule { };

        assert!(rule.apply(&mut board).unwrap());

        board.print();

        assert!(board.has_dot(0, 0));
        assert!(board.has_dot(0, 2));
        assert!(board.has_dot(0, 3));
        assert!(board.has_dot(0, 4));
        assert!(board.has_dot(1, 1));
        assert!(board.has_dot(2, 1));
        assert!(board.has_dot(3, 1));
        assert!(board.has_dot(4, 1));
    }
}