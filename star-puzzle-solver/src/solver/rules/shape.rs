use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use star_puzzle::board::{Board, ColorSection};
use crate::solver::rules::Rule;

// TODO: Don't use a hashmap, use a single struct or a 3rd parent struct so the rotation code doesn't have to be duplicated
// TODO List:
// 1. Allow negative numbers in the dot + star lists
// 2. Rotations and flips
// 3. Add shapes
pub struct ShapeRule {
    shapes: HashMap<Shape, ShapeResult>
}

impl ShapeRule {
    pub fn new() -> Self {
        Self {
            shapes: Self::rotate_shapes(HashMap::from([
                (Shape::from_positions(HashSet::from([(0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (1, 3)]), 2), ShapeResult::dots(vec![(0, 0)])), // 2x2 base with 2x2 L on top
                (Shape::from_positions(HashSet::from([(0, 0), (0, 1), (0, 2)]), 2), ShapeResult::stars(vec![(0, 0), (0, 2)])), // 1x3 2 star
            ]))
        }
    }

    fn rotate_shapes(shapes: HashMap<Shape, ShapeResult>) -> HashMap<Shape, ShapeResult> {
        let mut result = HashMap::new();

        // TODO: Clean this up
        shapes.into_iter().for_each(|(shape, shape_result)| {
            let rot_1 = shape.rotate();
            let out_rot_1 = shape_result.rotate(rot_1.1);
            let rot_2 = rot_1.0.rotate();
            let out_rot_2 = out_rot_1.rotate(rot_2.1);
            let rot_3 = rot_2.0.rotate();
            let out_rot_3 = out_rot_2.rotate(rot_3.1);

            result.insert(shape.clone(), shape_result.clone());
            result.insert(rot_1.0, out_rot_1);
            result.insert(rot_2.0, out_rot_2);
            result.insert(rot_3.0, out_rot_3);

            let shape = shape.flip();
            let rot_1 = shape.rotate();
            let out_rot_1 = shape_result.rotate(rot_1.1);
            let rot_2 = rot_1.0.rotate();
            let out_rot_2 = out_rot_1.rotate(rot_2.1);
            let rot_3 = rot_2.0.rotate();
            let out_rot_3 = out_rot_2.rotate(rot_3.1);

            result.insert(shape, shape_result);
            result.insert(rot_1.0, out_rot_1);
            result.insert(rot_2.0, out_rot_2);
            result.insert(rot_3.0, out_rot_3);
        });

        result
    }
}

impl Rule for ShapeRule {
    fn apply(&self, board: &mut Board) -> Result<bool, String> {
        let mut changed = false;
        for (i, color_section) in board.state.current_color_sections.clone().iter().enumerate() {
            let board_shape = Shape::from(color_section, board.max_star_count - *board.state.star_counts.entry(i).or_insert(0));
            for (shape, shape_result) in self.shapes.iter() {
                if board_shape.matches(shape) {
                    changed = shape_result.apply(&board_shape, board)? || changed;
                }
            }
        }

        Ok(changed)
    }

    fn name(&self) -> String {
        String::from("Shape Rule")
    }

    fn short_description(&self) -> String {
        String::from("Matches Shapes and places dots / stars in them")
    }
}

// Just represents a shape
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Shape {
    positions: HashSet<(usize, usize)>,
    offset: (usize, usize),
    required_star_count: usize
}

impl Shape {
    pub fn new(positions: HashSet<(usize, usize)>, offset: (usize, usize), required_star_count: usize) -> Self {
        Self { positions, offset, required_star_count }
    }

    pub fn from_positions(positions: HashSet<(usize, usize)>, required_star_count: usize) -> Self {
        Self {positions, offset: (0, 0), required_star_count }
    }

    pub fn from(color_section: &ColorSection, required_star_count: usize) -> Self {
        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        color_section.positions.iter().for_each(|pos| {
            let (x, y) = pos;
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
        });

        let offset = (min_x, min_y);

        let mut positions = HashSet::new();
        color_section.positions.iter().for_each(|pos| {
            positions.insert((pos.0 - offset.0, pos.1 - offset.1));
        });

        Shape::new(positions, offset, required_star_count)
    }

    pub fn matches(&self, other: &Shape) -> bool {
        self.positions == other.positions
    }

    /// Rotates the shape by 90 degrees.
    ///
    /// Returns the rotated shape and the max_y (used for rotating the ShapeResult).
    pub fn rotate(&self) -> (Self, usize) {
        let max_y = self.positions.iter().map(|(_, y)| *y).max().unwrap();
        let new_positions = self.positions.iter().map(|(x, y)| (max_y - *y, *x)).collect();

        (Shape::new(new_positions, self.offset, self.required_star_count), max_y)
    }

    /// Flips the shape
    pub fn flip(&self) -> Self {
        let max_y = self.positions.iter().map(|(_, y)| *y).max().unwrap();
        let new_positions = self.positions.iter().map(|(x, y)| (*x, max_y - *y)).collect();

        Shape::new(new_positions, self.offset, self.required_star_count)
    }
}

impl Hash for Shape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.positions.iter().collect::<Vec<_>>().sort().hash(state);
        self.required_star_count.hash(state);
    }
}

// Represents the dots / stars this shape requires
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ShapeResult {
    star_positions: Vec<(isize, isize)>,
    dot_positions: Vec<(isize, isize)>,
}

impl ShapeResult {
    pub fn new(star_positions: Vec<(isize, isize)>, dot_positions: Vec<(isize, isize)>) -> Self {
        Self { star_positions, dot_positions }
    }

    pub fn dots(dot_positions: Vec<(isize, isize)>) -> Self {
        Self::new(vec![], dot_positions)
    }

    pub fn stars(star_positions: Vec<(isize, isize)>) -> Self {
        Self::new(star_positions, vec![])
    }

    pub fn rotate(&self, max_y: usize) -> Self {
        let max_y = max_y as isize;
        let new_star_positions = self.star_positions.iter().map(|(x, y)| (max_y - *y, *x)).collect();
        let new_dot_positions = self.dot_positions.iter().map(|(x, y)| (max_y - *y, *x)).collect();

        Self::new(new_star_positions, new_dot_positions)
    }

    pub fn flip(&self, max_y: usize) -> Self {
        let max_y = max_y as isize;
        let new_star_positions = self.star_positions.iter().map(|(x, y)| (*x, max_y - *y)).collect();
        let new_dot_positions = self.dot_positions.iter().map(|(x, y)| (*x, max_y - *y)).collect();

        Self::new(new_star_positions, new_dot_positions)
    }

    fn calc_position(offset: (usize, usize), pos: &(isize, isize), board: &Board) -> Option<(usize, usize)> {
        let x = pos.0 + offset.0 as isize;
        let y = pos.1 + offset.1 as isize;

        if x < 0 || y < 0 {
            return None;
        }

        let x_usize = x as usize;
        let y_usize = y as usize;

        if board.in_range(x_usize, y_usize) {
            Some((x_usize, y_usize))
        } else {
            None
        }
    }

    pub fn apply(&self, shape: &Shape, board: &mut Board) -> Result<bool, String> {
        let offset = shape.offset;
        let mut changed = false;
        for pos in self.star_positions.iter() {
            match Self::calc_position(offset, pos, board) {
                Some(new_pos) => {
                    if !board.has_star(new_pos.0, new_pos.1) {
                        board.place_star(new_pos.0, new_pos.1)?;
                        changed = true;
                    }
                }
                None => {}
            }
        }

        for pos in self.dot_positions.iter() {
            match Self::calc_position(offset, pos, board) {
                Some(new_pos) => {
                    changed = board.place_dot(new_pos.0, new_pos.1) || changed;
                }
                None => {}
            }
        }

        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_shape_works() {
        let mut board = Board::from_string("aaccchhhhg\naaachhhggg\naaacdhgggg\naaacddiggg\naaccdiieee\nfcccdiieee\nffccdiieje\nfbbbbeeejj\nffbbeeejjj\nffbbbeejjj", 1).unwrap();

        let rule = ShapeRule::new();
        let changed = rule.apply(&mut board).unwrap();
        board.print();
        assert!(changed);
        assert!(board.has_dot(5, 3));
    }

    #[test]
    fn three_by_one_still_works() {
        let mut board = Board::from_string("0111222222\n0333332222\n0300332422\n0005552422\n0000000422\n0000222222\n0000067772\n0088862222\n6666669992\n6666666222", 2).unwrap();
        println!("Board to solve:");
        board.print();

        let rule = ShapeRule::new();

        let result = rule.apply(&mut board);

        println!("Solved Board:");
        board.print();

        assert!(result.unwrap());
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

        assert!(!second_result.unwrap());
    }
}