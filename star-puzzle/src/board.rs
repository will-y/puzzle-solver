use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use colored::{Color, Colorize};

pub mod data;
pub mod state;

const COLORS: [Color; 10] = [Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::White, Color::BrightRed, Color::BrightGreen, Color::BrightBlue, Color::BrightCyan];

#[derive(PartialEq, Debug)]
pub struct Board {
    color_sections: Vec<ColorSection>,
    star_count: usize,
    max_star_count: usize,
    state: State,
    size: usize,
}

impl Board {
    /// Creates a new board from a string and a star count.
    /// The following basic validations are performed:
    /// - Board is a square
    /// - All color sections are continuous
    ///
    /// # Examples
    /// ```
    /// use star_puzzle::board::Board;
    ///
    /// let board = Board::from_string("00011\n21113\n21433\n44433\n44444", 1).unwrap();
    ///
    /// assert!(board.is_valid());
    /// ```
    pub fn from_string(s: &str, max_star_count: usize) -> Result<Board, String> {
        let board_size = s.lines().count();

        let mut color_section_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                color_section_map
                    .entry(c)
                    .or_insert(HashSet::new())
                    .insert((x, y));
            });
        });

        let color_sections = color_section_map
            .values()
            .map(|positions| ColorSection {
                positions: positions.clone(),
                star_count: 0,
            })
            .collect();

        Ok(Board {
            color_sections,
            star_count: 0,
            max_star_count,
            state: State::new(),
            size: board_size
        })
    }

    // TODO: Do I need this or should it validate moves in-place?
    pub fn is_valid(&self) -> bool {
        // Rows
        // Columns
        // Sections
        // Adjacent
        true
    }

    pub fn is_solved(&self) -> bool {
        let colors_valid = self
            .color_sections
            .iter()
            .any(|color_section| color_section.star_count != self.max_star_count);

        if !colors_valid {
            return false;
        }

        let mut row_counts = vec![0; self.color_sections.len()];
        let mut col_counts = vec![0; self.color_sections.len()];

        self.state.star_placements.iter().for_each(|placement| {
            row_counts[placement.0] += 1;
            col_counts[placement.1] += 1;
        });

        !row_counts.iter().any(|x| *x != self.max_star_count)
            && !col_counts.iter().any(|x| *x != self.max_star_count)
    }

    pub fn place_star(&mut self, x: usize, y: usize) {
        self.state.star_placements.push((x, y));
    }

    pub fn has_star(&self, x: usize, y: usize) -> bool {
        self.state.star_placements.contains(&(x, y))
    }

    pub fn print(&self) {
        let mut color_map = vec![vec!(0; self.size); self.size];

        self.color_sections
            .iter()
            .enumerate()
            .for_each(|(i, section)| {
                section.positions.iter().for_each(|(x, y)| {
                    color_map[*y][*x] = i;
                });
            });

        color_map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, c)| {
                if self.has_star(x, y) {
                    print!("{ }", "*".color(COLORS[*c]));
                } else if self.state.dot_placements.contains(&(x, y)) {
                    print!("{ }", ".".color(COLORS[*c]));
                } else {
                    print!("{ }", "□".color(COLORS[*c]));
                }
            });
            println!();
        });
    }
}

#[derive(PartialEq, Debug)]
pub struct ColorSection {
    positions: HashSet<(usize, usize)>,
    star_count: usize,
}

#[derive(PartialEq, Debug)]
pub struct State {
    star_placements: Vec<(usize, usize)>,
    dot_placements: Vec<(usize, usize)>,
}

impl State {
    pub fn new() -> State {
        let star_placements = vec![];
        let dot_placements = vec![];
        State { star_placements, dot_placements }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved_simple_board() {
        let mut board = Board::from_string("00011\n21113\n21433\n44433\n44444", 1).unwrap();

        board.print();

        board.place_star(0, 2);
        board.place_star(1, 4);
        board.place_star(2, 0);
        board.place_star(3, 3);
        board.place_star(4, 1);

        assert!(board.is_solved());
    }

    #[test]
    fn is_solved_2_stars() {
        let mut board = Board::from_string("0111222222\n0333332222\n0300332422\n0005552422\n0000000422\n0000222222\n0000067772\n0088862222\n6666669992\n66666662222", 2).unwrap();

        board.print();
    }
}
