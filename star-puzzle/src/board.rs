use std::collections::{HashMap, HashSet};
use colored::{Color, Colorize};

const COLORS: [Color; 10] = [Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::White, Color::BrightRed, Color::BrightGreen, Color::BrightBlue, Color::BrightCyan];

#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    color_sections: Vec<ColorSection>,
    star_count: usize,
    max_star_count: usize,
    state: State,
    pub size: usize,
    row_counts: Vec<usize>, // TODO: should these move to state? Mutable data is also stored in color_sections now ...
    col_counts: Vec<usize>
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
            size: board_size,
            row_counts: vec![0; board_size],
            col_counts: vec![0; board_size]
        })
    }

    pub fn is_solved(&self) -> bool {
        let colors_valid = self
            .color_sections
            .iter()
            .all(|color_section| color_section.star_count == self.max_star_count);

        if !colors_valid {
            return false;
        }

        !self.row_counts.iter().any(|x| *x != self.max_star_count)
            && !self.col_counts.iter().any(|x| *x != self.max_star_count)
    }
    
    pub fn place_star(&mut self, x: usize, y: usize) -> Result<(), String> {
        if self.in_range(x, y) {
            self.state.star_placements.push((x, y));

            // Row / Col counts are not correct
            if self.row_counts[y] + 1 > self.max_star_count {
                return Err("Too many stars in this column".to_string())
            }

            if self.col_counts[x] + 1 > self.max_star_count {
                return Err("Too many stars in this row".to_string())
            }

            self.row_counts[y] += 1;
            self.col_counts[x] += 1;
            
            for color_section in self.color_sections.iter_mut() {
                if color_section.positions.contains(&(x, y)) {
                    if color_section.star_count + 1 > self.max_star_count {
                        return Err("Too many stars in this color section".to_string())
                    }
                    
                    color_section.star_count += 1;
                }
            }

            let mut surrounding: Vec<(usize, usize)> = vec!();

            for i in -1..2 {
                for j in -1..2 {
                    if i != 0 || j != 0 {
                        let new_x: isize = x as isize + i;
                        let new_y: isize = y as isize + j;

                        if new_x >= 0 && new_y >= 0 {
                            surrounding.push((new_x as usize, new_y as usize));
                        }
                    }
                }
            }

            if surrounding.iter().any(|pos| self.has_star(pos.0, pos.1)) {
                return Err("Cannot place star next to another star".to_string())
            }

            surrounding.iter().for_each(|(x, y)| {
                self.place_dot(*x, *y);
            });

            return Ok(())
        }

        Err("Invalid position".to_string())
    }

    pub fn place_dot(&mut self, x: usize, y: usize) {
        if self.in_range(x, y) && !self.has_star(x, y) {
            self.state.dot_placements.push((x, y));
        }
    }

    pub fn has_star(&self, x: usize, y: usize) -> bool {
        self.state.star_placements.contains(&(x, y))
    }

    pub fn has_dot(&self, x: usize, y: usize) -> bool {
        self.state.dot_placements.contains(&(x, y))
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        !self.has_star(x, y) && !self.has_dot(x, y)
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

    fn in_range(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ColorSection {
    positions: HashSet<(usize, usize)>,
    star_count: usize,
}

#[derive(PartialEq, Debug, Clone)]
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
