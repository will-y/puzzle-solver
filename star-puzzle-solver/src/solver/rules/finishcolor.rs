use crate::solver::rules::Rule;
use star_puzzle::board::{Board};

/// This rule does 2 things:
/// 1. Fills in full colors with dots
/// 2. Fills in stars if there are only enough empty spaces for colors
pub struct FinishColorRule {}

impl FinishColorRule {
    fn fill_color_dots(&self, board: &mut Board) -> bool {
        let mut changed = false;
        let positions = board
            .state
            .current_color_sections
            .iter()
            .enumerate()
            .flat_map(|(i, section)| {
                if *(board.state.star_counts.get(&i).unwrap_or(&0)) == board.max_star_count
                    && section.positions.len() > 0 {
                    changed = true;
                    return section.positions.iter()
                        .map(|(x, y)| (*x, *y))
                        .collect::<Vec<(usize, usize)>>();
                }

                return vec![];
            })
            .collect::<Vec<(usize, usize)>>();

        positions.iter().for_each(|(x, y)| {
            board.place_dot(*x, *y);
        });

        changed
    }

    fn fill_color_stars(&self, board: &mut Board) -> bool {
        let mut changed = false;
        let positions: Vec<(usize, usize)> = board
            .state
            .current_color_sections
            .iter()
            .enumerate()
            .flat_map(|(i, section)| {
                if board.max_star_count - *board.state.star_counts.get(&i).unwrap_or(&0)
                    == section.positions.len()
                    && section.positions.len() > 0 {
                    changed = true;
                    return section
                        .positions
                        .iter()
                        .map(|(x, y)| (*x, *y))
                        .collect::<Vec<(usize, usize)>>();
                }

                return vec![];
            })
            .collect::<Vec<(usize, usize)>>();

        positions.iter().for_each(|(x, y)| {
            board.place_star(*x, *y).expect("Could not place star in finish color rule");
        });

        changed
    }
}

impl Rule for FinishColorRule {
    fn apply(&self, board: &mut Board) -> bool {
        let dots = self.fill_color_dots(board);
        let stars = self.fill_color_stars(board);

        dots || stars
    }

    fn name(&self) -> String {
        String::from("Finish Color")
    }

    fn short_description(&self) -> String {
        String::from("Fills in full colors with dots and places stars if there equal empty spaces and required stars")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_fill_in_section_with_stars() {
        let mut board = Board::from_string("0111222222\n0333332222\n0300332422\n0005552422\n0000000422\n0000222222\n0000067772\n0088862222\n6666669992\n6666666222", 2).unwrap();
        board.place_dot(2, 0);

        let rule = FinishColorRule {};

        assert!(rule.apply(&mut board));

        board.print();

        assert!(board.has_star(1, 0));
        assert!(board.has_star(3, 0));
    }
}