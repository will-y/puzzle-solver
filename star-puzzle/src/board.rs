use colored::{Color, Colorize};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

const COLORS: [Color; 11] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::White,
    Color::BrightRed,
    Color::BrightGreen,
    Color::BrightBlue,
    Color::BrightCyan,
    Color::BrightMagenta,
];

#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    pub color_sections: Vec<ColorSection>,
    pub max_star_count: usize,
    pub size: usize,
    pub state: State,
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
    /// assert!(board.is_empty(0, 0));
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

        let color_sections: Vec<ColorSection> = color_section_map
            .values()
            .map(|positions| ColorSection {
                positions: positions.clone(),
            })
            .collect();

        let initial_color_sections = color_sections.clone();

        Ok(Board {
            color_sections,
            max_star_count,
            state: State::new(board_size, initial_color_sections),
            size: board_size,
        })
    }

    pub fn from_color_map(
        color_map: &Vec<Vec<usize>>,
        max_star_count: usize,
    ) -> Result<Board, String> {
        let board_size = color_map.len();
        let mut color_section_map: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();

        color_map.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, c)| {
                color_section_map
                    .entry(*c)
                    .or_insert(HashSet::new())
                    .insert((x, y));
            });
        });

        // TODO: Combine this part with the one above
        let color_sections: Vec<ColorSection> = color_section_map
            .values()
            .map(|positions| ColorSection {
                positions: positions.clone(),
            })
            .collect();

        let initial_color_sections = color_sections.clone();

        Ok(Board {
            color_sections,
            max_star_count,
            state: State::new(board_size, initial_color_sections),
            size: board_size,
        })
    }

    pub fn is_solved(&self) -> bool {
        let colors_valid = self
            .state
            .star_counts
            .iter()
            .all(|(_, count)| *count == self.max_star_count);

        if !colors_valid {
            return false;
        }

        !self
            .state
            .row_counts
            .iter()
            .any(|x| *x != self.max_star_count)
            && !self
                .state
                .col_counts
                .iter()
                .any(|x| *x != self.max_star_count)
    }

    pub fn place_star(&mut self, x: usize, y: usize) -> Result<(), String> {
        // TODO: Consider moving more of this to state
        if self.has_dot(x, y) {
            return Err(format!(
                "Cannot place a dot on top of a star (placing star at ({ }, { })",
                x, y
            ));
        }
        if self.in_range(x, y) {
            // Row / Col counts are not correct
            if self.state.row_counts[y] + 1 > self.max_star_count {
                return Err(format!(
                    "Too many stars in column { } (placing star at ({ }, { })",
                    y, x, y
                ));
            }

            if self.state.col_counts[x] + 1 > self.max_star_count {
                return Err(format!(
                    "Too many stars in row { } (placing star at ({ }, { })",
                    x, x, y
                ));
            }

            let mut surrounding: Vec<(usize, usize)> = vec![];

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
                let conflicting = surrounding
                    .iter()
                    .find(|pos| self.has_star(pos.0, pos.1))
                    .unwrap();
                return Err(format!(
                    "Cannot place star next to another star. Placing: ({ }, { }) Existing: ({ }, { })",
                    x, y, conflicting.0, conflicting.1
                ));
            }

            self.state.star_placements.insert((x, y));
            self.state.place_star(x, y, self.max_star_count)?;

            surrounding.iter().for_each(|(x, y)| {
                self.place_dot(*x, *y);
            });

            return Ok(());
        }

        Err(format!("Invalid position ({ }, { }", x, y))
    }

    /// Places a dot on the board.
    ///
    /// Returns true if the dot was actually placed (not placed on a star or dot)
    pub fn place_dot(&mut self, x: usize, y: usize) -> bool {
        if self.in_range(x, y) && !self.has_star(x, y) {
            return self.state.place_dot(x, y);
        }

        false
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

    pub fn create_color_map(&self) -> Vec<Vec<usize>> {
        let mut color_map = vec![vec!(0; self.size); self.size];

        self.color_sections
            .iter()
            .enumerate()
            .for_each(|(i, section)| {
                section.positions.iter().for_each(|(x, y)| {
                    color_map[*y][*x] = i;
                });
            });

        color_map
    }

    pub fn has_contradictions(&self) -> bool {
        // TODO: Need more things here?
        self
            .state
            .current_color_sections
            .iter()
            .enumerate()
            .any(|(i, section)| {
                section.positions.len() < self.max_star_count - *self.state.star_counts.get(&i).unwrap_or(&0)
            })
    }

    pub fn print(&self) {
        let color_map = self.create_color_map();

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

    pub fn to_string(&self) -> String {
        let mut result = vec![vec!['0'; self.size]; self.size];

        self.color_sections
            .iter()
            .enumerate()
            .for_each(|(i, section)| {
                section.positions.iter().for_each(|(x, y)| {
                    result[*y][*x] = char::from_u32((i + 97) as u32).unwrap();
                });
            });

        result
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn in_range(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ColorSection {
    pub positions: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub star_placements: HashSet<(usize, usize)>,
    pub dot_placements: HashSet<(usize, usize)>,
    pub row_counts: Vec<usize>,
    pub col_counts: Vec<usize>,
    pub current_color_sections: Vec<ColorSection>,
    pub star_counts: HashMap<usize, usize>, // Map of color section index to star count in that section
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.star_placements == other.star_placements && self.dot_placements == other.dot_placements
    }
}

impl Eq for State {}

impl Hash for State {
    // Gross but whatever
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut stars: Vec<&(usize, usize)> = self.star_placements.iter().collect();
        stars.sort();
        stars.hash(state);

        let mut dots: Vec<&(usize, usize)> = self.dot_placements.iter().collect();
        dots.sort();
        dots.hash(state);
    }
}

impl State {
    pub fn new(board_size: usize, initial_color_sections: Vec<ColorSection>) -> State {
        let star_placements = HashSet::new();
        let dot_placements = HashSet::new();

        State {
            star_placements,
            dot_placements,
            row_counts: vec![0; board_size],
            col_counts: vec![0; board_size],
            current_color_sections: initial_color_sections,
            star_counts: HashMap::new(),
        }
    }

    pub fn place_star(&mut self, x: usize, y: usize, max_star_count: usize) -> Result<(), String> {
        for (i, color_section) in self.current_color_sections.iter_mut().enumerate() {
            if color_section.positions.contains(&(x, y)) {
                if *self.star_counts.entry(i).or_insert(0) + 1 > max_star_count {
                    return Err("Too many stars in this color section".to_string());
                }

                *self.star_counts.entry(i).or_insert(0) += 1;
                color_section.positions.remove(&(x, y));
            }
        }

        self.row_counts[y] += 1;
        self.col_counts[x] += 1;

        Ok(())
    }

    pub fn place_dot(&mut self, x: usize, y: usize) -> bool {
        let inserted = self.dot_placements.insert((x, y));

        if inserted {
            for color_section in self.current_color_sections.iter_mut() {
                color_section.positions.remove(&(x, y));
            }
        }

        inserted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved_simple_board() {
        let mut board = Board::from_string("00011\n21113\n21433\n44433\n44444", 1).unwrap();

        board.print();

        board.place_star(0, 2).unwrap();
        board.place_star(1, 0).unwrap();
        board.place_star(2, 4).unwrap();
        board.place_star(3, 1).unwrap();
        board.place_star(4, 3).unwrap();

        board.print();

        assert!(board.is_solved());
    }
}
