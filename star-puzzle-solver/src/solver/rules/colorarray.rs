use star_puzzle::board::{Board, ColorSection};
use crate::solver::rules::Rule;

pub struct ColorArrayRule {}

impl ColorArrayRule {
    fn apply_rows(&self, board: &mut Board) -> Result<bool, String> {
        let mut changed = false;
        // 1. Check if a row is entirely one color
        for row in 0..board.size {
            // Maybe something smarter to do here? Check row needed stars vs color needed stars?
            if board.state.row_counts[row] == 0 {
                let sections_in_row: Vec<ColorSection> = board.state.current_color_sections.clone().into_iter()
                    .filter(|section| section.positions.iter().any(|pos| pos.1 == row))
                    .collect();

                if sections_in_row.len() == 1 {
                    // Only one color in this row and the row needs full stars
                    for pos in sections_in_row[0].positions.iter() {
                        if pos.1 != row {
                            changed = board.place_dot(pos.0, pos.1) || changed;
                        }
                    }
                }
            }
        }

        // 2. Check if a color is entirely contained in one row
        for (i, color_section) in board.state.current_color_sections.clone().iter().enumerate() {
            if color_section.positions.len() >  0 {
                let row = color_section.positions.iter().last().unwrap().1;
                if color_section.positions.iter().all(|pos| pos.1 == row) {
                    let row_count = board.state.row_counts[row];
                    let section_count = *board.state.star_counts.entry(i).or_insert(0);

                    if row_count == section_count {
                        for x in 0..board.size {
                            if !color_section.positions.contains(&(x, row)) {
                                changed = board.place_dot(x, row) || changed;
                            }
                        }
                    }
                }
            }
        }

        Ok(changed)
    }

    fn apply_columns(&self, board: &mut Board) -> Result<bool, String> {
        let mut changed = false;

        for col in 0..board.size {
            if board.state.col_counts[col] == 0 {
                let sections_in_col: Vec<ColorSection> = board.state.current_color_sections.clone().into_iter()
                    .filter(|section| section.positions.iter().any(|pos| pos.0 == col))
                    .collect();

                if sections_in_col.len() == 1 {
                    // Only one color in this column and the column needs full stars
                    for pos in sections_in_col[0].positions.iter() {
                        if pos.0 != col {
                            changed = board.place_dot(pos.0, pos.1) || changed;
                        }
                    }
                }
            }
        }

        // 2. Check if a color is entirely contained in one column
        for (i, color_section) in board.state.current_color_sections.clone().iter().enumerate() {
            if color_section.positions.len() > 0 {
                let col = color_section.positions.iter().last().unwrap().0;
                if color_section.positions.iter().all(|pos| pos.0 == col) {
                    let col_count = board.state.col_counts[col];
                    let section_count = *board.state.star_counts.entry(i).or_insert(0);

                    if col_count == section_count {
                        for y in 0..board.size {
                            if !color_section.positions.contains(&(col, y)) {
                                changed = board.place_dot(col, y) || changed;
                            }
                        }
                    }
                }
            }
        }

        Ok(changed)
    }
}

impl Rule for ColorArrayRule {
    fn apply(&self, board: &mut Board) -> Result<bool, String> {
        let row = self.apply_rows(board)?;
        let column = self.apply_columns(board)?;

        Ok(row || column)
    }

    fn name(&self) -> String {
        String::from("Color Array")
    }

    fn short_description(&self) -> String {
        String::from("Does two things. First it checks if a row or column is entirely one color, if so it fills in the rest of the color. Second it checks if a color is entirely contained in one row / col. If so it fills in the rest of the row /col.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fills_color() {
        let mut board = Board::from_string("1111111111\n1111110000\n110200000\n1402220000\n0444420000\n0333000000\n0300000000\n0500000000\n0500000000\n0550000000", 2).unwrap();
        println!("Before: ");
        board.print();

        let rule = ColorArrayRule{};

        assert!(rule.apply(&mut board).unwrap());

        println!("After: ");
        board.print();
    }

    #[test]
    fn it_fills_if_one_color_entirely_contained() {
        let mut board = Board::from_string("0010000000\n0010000000\n0010000000\n0010000000\n0010000000\n0010000000\n5000333330\n5000000000\n5222220044\n5550004444", 2).unwrap();
        board.place_star(4, 6).unwrap();
        println!("Before: ");
        board.print();

        let rule = ColorArrayRule{};

        assert!(rule.apply(&mut board).unwrap());

        println!("After: ");
        board.print();
    }
}