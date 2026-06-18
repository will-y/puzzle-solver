use std::collections::HashSet;
use star_puzzle::board::Board;
use crate::solver::rules::Rule;

/// This rule fills in a row or column if it has an equal number of required stars and empty spaces.
/// TODO: Make this smarter and able to solve more complex row / col only scenarios
/// Ex: with 3 stars ..._____... = ...*.*.*...
/// or with 2 stars ..__..._. = ..__...*.
pub struct FillArrayRule {}

impl FillArrayRule {
    fn collect_empty_indices(&self, board: &Board, index: usize, row: bool) -> Vec<usize> {
        let mut indices = (0..board.size).collect::<HashSet<usize>>();

        board.state.star_placements.iter()
            .chain(board.state.dot_placements.iter())
            .filter(|pos| if row { pos.1 } else { pos.0 } == index)
            .for_each(|pos| {
                indices.remove(if row { &pos.0 } else { &pos.1 });
            });

        indices.into_iter().collect::<Vec<usize>>()
    }

    fn fill_rows(&self, board: &mut Board) -> Result<bool, String> {
        let mut changed = false;

        for i in 0..board.size {
            let available_stars = board.max_star_count - board.state.row_counts[i];
            if available_stars > 0 {
                let empty_indices = self.collect_empty_indices(board, i, true);

                if empty_indices.len() == available_stars {
                    for j in empty_indices {
                        board.place_star(j, i)?
                    }

                    changed = true;
                }
            }
        }

        Ok(changed)
    }

    fn fill_columns(&self, board: &mut Board) -> Result<bool, String> {
        let mut changed = false;

        for i in 0..board.size {
            let available_stars = board.max_star_count - board.state.col_counts[i];
            if available_stars > 0 {
                let empty_indices = self.collect_empty_indices(board, i, false);

                if empty_indices.len() == available_stars {
                    for j in empty_indices {
                        board.place_star(i, j)?;
                    }

                    changed = true;
                }
            }
        }

        Ok(changed)
    }
}

impl Rule for FillArrayRule {
    fn apply(&self, board: &mut Board) -> Result<bool, String> {
        let rows_filled = self.fill_rows(board)?;
        let columns_filled = self.fill_columns(board)?;

        Ok(rows_filled || columns_filled)
    }

    fn name(&self) -> String {
        String::from("Place Star in Row / Column")
    }

    fn short_description(&self) -> String {
        String::from("Places stars in a row or column if it has an equal number of required stars and empty spaces")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_place_stars_in_correct_spots() {
        let mut board = Board::from_string("0111222222\n0333332222\n0300332422\n0005552422\n0000000422\n0000222222\n0000067772\n0088862222\n6666669992\n6666666222", 2).unwrap();

        board.place_dot(0, 1);
        board.place_dot(0, 2);
        board.place_dot(0, 3);
        board.place_dot(0, 4);
        board.place_dot(0, 6);
        board.place_dot(0, 7);
        board.place_dot(0, 8);
        board.place_dot(0, 9);

        board.place_dot(1, 0);
        board.place_dot(2, 0);
        board.place_dot(3, 0);
        board.place_dot(5, 0);
        board.place_dot(6, 0);
        board.place_dot(7, 0);
        board.place_dot(8, 0);
        board.place_dot(9, 0);

        let rule = FillArrayRule { };

        assert!(rule.apply(&mut board).unwrap());

        board.print();

        assert!(board.has_star(0, 0));
        assert!(board.has_star(0, 5));
        assert!(board.has_star(4, 0));

    }
}