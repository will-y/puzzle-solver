use crate::solver::rules::Rule;
use star_puzzle::board::Board;

/// Basic 2-star rule:
/// If there is a row or column of 3 empty spaces, 2 stars can be placed there
pub struct LineRule2Star {}

impl Rule for LineRule2Star {
    fn apply(&self, board: &mut Board) -> bool {
        let stars_to_add = board
            .state
            .current_color_sections
            .iter_mut()
            .enumerate()
            .flat_map(|(i, section)| {
                if section.positions.len() == 3
                    && *board.state.star_counts.entry(i).or_insert(0) == 0 {
                    let rand_element = section.positions.iter().next().unwrap();

                    return if section.positions.iter().all(|pos| pos.0 == rand_element.0) {
                        // All x the same, place at smallest and largest Y
                        let min = section.positions.iter().min_by_key(|pos| pos.1).unwrap();
                        let max = section.positions.iter().max_by_key(|pos| pos.1).unwrap();

                        vec![(rand_element.0, min.1), (rand_element.0, max.1)]
                    } else if section.positions.iter().all(|pos| pos.1 == rand_element.1) {
                        // All y the same, place at smallest and largest X
                        let min = section.positions.iter().min_by_key(|pos| pos.0).unwrap();
                        let max = section.positions.iter().max_by_key(|pos| pos.0).unwrap();

                        vec![(min.0, rand_element.1), (max.0, rand_element.1)]
                    } else {
                        vec![]
                    }
                }

                vec![]
            }).collect::<Vec<(usize, usize)>>();

        stars_to_add.iter().for_each(|position| {
            board.place_star(position.0, position.1).expect("Could not place star in line rule 2 star rule");
        });

        stars_to_add.len() > 0
    }

    fn can_apply(&self, board: &Board) -> bool {
        board.max_star_count == 2
    }

    fn name(&self) -> String {
        String::from("3x1 Shape Rule (2-star only)")
    }

    fn short_description(&self) -> String {
        String::from("Places 2 stars in a row or column if there are 3 empty spaces in a row / column")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_apply_stars() {
        let mut board = Board::from_string("0111222222\n0333332222\n0300332422\n0005552422\n0000000422\n0000222222\n0000067772\n0088862222\n6666669992\n6666666222", 2).unwrap();

        let rule = LineRule2Star {};

        let result = rule.apply(&mut board);

        board.print();

        assert!(result);
        assert!(board.has_star(1, 0));
        assert!(board.has_star(3, 0));
        assert!(board.has_star(3, 3));
        assert!(board.has_star(5, 3));
        assert!(board.has_star(6, 6));
        assert!(board.has_star(8, 6));
        assert!(board.has_star(2, 7));
        assert!(board.has_star(4, 7));
        assert!(board.has_star(6, 8));
        assert!(board.has_star(8, 8));
        assert!(board.has_star(7, 2));
        assert!(board.has_star(7, 4));

        // Make sure a second run doesn't do anything
        let second_result = rule.apply(&mut board);

        assert!(!second_result);
    }
}
