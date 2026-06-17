use crate::board::Board;
use itertools::Itertools;
use rand::prelude::{IteratorRandom, SliceRandom};
use std::collections::{HashMap, HashSet};
use utilities::astar;

/// Generates a board with `max_star_count` stars.
/// The size will be `max_star_count` * 5.
///
/// `difficulty` is a float from 0-1 that determines how hard the generated puzzle will be.
///
/// # Example
/// ```
/// use star_puzzle::generator::generate_board;
/// let board = generate_board(2).unwrap();
/// board.print();
/// ```
pub fn generate_board(max_star_count: usize, difficulty: f32) -> Result<Board, String> {
    validate_arguments(max_star_count, difficulty)?;

    let board_size = max_star_count * 5;
    let mut attempts = 0;

    while attempts < 100 {
        // 1. Randomly Place Stars
        let star_placements = get_random_star_placements(max_star_count, board_size);
        println!("{:?}", star_placements);
        // 2. Seed color sections
        let mut color_map = vec![vec![0; board_size]; board_size];
        let star_groups = seed_color_map(&mut color_map, star_placements, max_star_count);
        println!("{:?}", color_map);
        // 3. Connect color sections
        match connect_color_map(&mut color_map, &star_groups) {
            Ok(_) => {}
            Err(_) => {
                attempts += 1;
                continue;
            },
        }
        let board = Board::from_color_map(&color_map, max_star_count)?;
        println!("Board Before Flood Fill:");
        board.print();
        // 4. Flood fill color sections
        flood_fill_color_map(&mut color_map, board_size, difficulty);
        let board = Board::from_color_map(&color_map, max_star_count)?;
        println!("Board After Flood Fill:");
        board.print();
        return Ok(board)
    }

    Err("Failed to generate board after 100 attempts".to_string())
}

fn validate_arguments(max_star_count: usize, difficulty: f32) -> Result<(), String> {
    if max_star_count > 5 {
        return Err("Star count must be at most 5".to_string());
    }

    if difficulty < 0.0 || difficulty > 1.0 {
        return Err("Difficulty must be between 0 and 1".to_string());
    }

    Ok(())
}

fn get_random_star_placements(max_star_count: usize, board_size: usize) -> HashSet<(usize, usize)> {
    let initial_state = State {
        x_coords: vec![],
        col_counts: vec![0; board_size],
    };

    let solved_state = get_random_star_placements_backtrace(initial_state, max_star_count)
        .expect("No solution found");

    let mut result: HashSet<(usize, usize)> = HashSet::new();

    solved_state
        .x_coords
        .iter()
        .enumerate()
        .for_each(|(y, x_coords)| {
            x_coords.iter().for_each(|x| {
                result.insert((*x, y));
            });
        });

    result
}

fn get_random_star_placements_backtrace(state: State, max_star_count: usize) -> Option<State> {
    if state.col_counts.iter().all(|x| *x == max_star_count) {
        return Some(state);
    }

    for choice in all_random_indices_from(
        &state.col_counts,
        max_star_count,
        get_total_not_allowed(state.x_coords.last().unwrap_or(&vec![])),
    ) {
        if !is_valid_choice(&choice) {
            continue;
        }
        let mut next_state = State {
            x_coords: state.x_coords.clone(),
            col_counts: state.col_counts.clone(),
        };

        for i in choice.iter() {
            next_state.col_counts[*i] += 1;
        }
        next_state.x_coords.push(choice);

        match get_random_star_placements_backtrace(next_state, max_star_count) {
            Some(s) => return Some(s),
            None => (),
        };
    }

    None
}

fn is_valid_choice(choice: &Vec<usize>) -> bool {
    let mut cloned_choices = choice.clone();
    cloned_choices.sort();

    cloned_choices.windows(2).all(|w| w[0] + 1 != w[1])
}

fn get_total_not_allowed(last_x_coords: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![];

    for x in last_x_coords {
        result.push(*x);
        result.push(*x + 1);
        if *x > 0 {
            result.push(*x - 1);
        }
    }

    result
}

fn all_random_indices_from(
    col_counts: &Vec<usize>,
    max_star_count: usize,
    not_allowed: Vec<usize>,
) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = col_counts
        .iter()
        .enumerate()
        .filter(|(_, count)| **count < max_star_count)
        .map(|(i, _)| i)
        .filter(|i| !not_allowed.contains(i))
        .combinations(max_star_count)
        .collect();

    result.shuffle(&mut rand::thread_rng());

    result
}

fn seed_color_map(
    color_map: &mut Vec<Vec<usize>>,
    stars: HashSet<(usize, usize)>,
    max_star_count: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut star_copy = stars.iter().collect::<Vec<_>>();
    star_copy.shuffle(&mut rand::thread_rng());

    let mut star_groups: Vec<Vec<(usize, usize)>> = vec![];

    let mut used_stars = HashSet::new();
    let mut color_index = 1;

    while let Some(star) = star_copy.pop() {
        if used_stars.insert(star) {
            let mut star_group = vec![*star];
            color_map[star.1][star.0] = color_index;
            star_copy
                .iter()
                .filter(|star| !used_stars.contains(*star))
                .sorted_by(|a, b| {
                    ((a.0 as isize - star.0 as isize).pow(2)
                        + (a.1 as isize - star.1 as isize).pow(2))
                    .cmp(
                        &((b.0 as isize - star.0 as isize).pow(2)
                            + (b.1 as isize - star.1 as isize).pow(2)),
                    )
                })
                .take(max_star_count - 1)
                .for_each(|grouped_star| {
                    color_map[grouped_star.1][grouped_star.0] = color_index;
                    color_index += 1;
                    used_stars.insert(grouped_star);
                    star_group.push(**grouped_star);
                });

            star_groups.push(star_group);
        }
    }

    star_groups
}

fn connect_color_map(color_map: &mut Vec<Vec<usize>>, groups: &Vec<Vec<(usize, usize)>>) -> Result<(), String> {
    let mut attempt = 0;

    while attempt < 100 {
        let mut temp_color_map = color_map.clone();
        let mut range = (1..=groups.len()).collect::<Vec<_>>();
        let mut color_map_positions = HashMap::new();
        let mut solved = true;

        range.shuffle(&mut rand::thread_rng());
        for i in range.into_iter() {
            match run_a_star_for_color_map(&temp_color_map, i, &groups[i - 1]) {
                Some(path) => {
                    for point in path.iter() {
                        color_map_positions.insert(point.clone(), i);
                        temp_color_map[point.1][point.0] = i;
                    }

                    println!("After A* Attempt { }, Color { } {:?}", attempt, i,path);
                }
                None => {
                    attempt += 1;
                    solved = false;
                    break;
                }
            }
        }
        if solved {
            color_map_positions.iter().for_each(|(point, i)| {
                color_map[point.1][point.0] = *i;
            });
            return Ok(())
        }
    }

    Err("Failed to connect star groups".to_string())
}

// TODO: This only works for 2-star boards RN
fn run_a_star_for_color_map(
    color_map: &Vec<Vec<usize>>,
    color_index: usize,
    star_positions: &Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let start = star_positions[0];
    let end_condition = |pos, value| pos != start && value == color_index;
    let heuristic = |_, _| 0;
    let valid_space = |_, value| value == 0 || value == color_index;
    astar::run_board_a_star(color_map, start, end_condition, heuristic, valid_space)
}

fn flood_fill_color_map(color_map: &mut Vec<Vec<usize>>, board_size: usize, difficulty: f32) {
    // 1. Get a set of all uncolored positions
    let difficulty_color_count = ((difficulty * board_size as f32).round() as usize).clamp(1, board_size);

    let difficulty_colors = (1..=board_size).into_iter()
        .choose_multiple(&mut rand::thread_rng(), difficulty_color_count)
        .into_iter()
        .collect::<HashSet<_>>();

    let mut uncolored_positions: HashSet<(usize, usize)> = HashSet::new();
    color_map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c == 0 {
                uncolored_positions.insert((x, y));
            }
        });
    });

    while uncolored_positions.len() > 0 {
        let mut uncolored_positions_next_to_colored: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        uncolored_positions.iter().for_each(|original_pos| {
            let neighbors = astar::neighbors(*original_pos, color_map[0].len(), color_map.len());
            neighbors.iter()
                .filter(|pos| difficulty_colors.contains(&color_map[pos.1][pos.0]))
                .for_each(|pos| {
                    uncolored_positions_next_to_colored.entry(*original_pos).or_insert(vec![]).push(color_map[pos.1][pos.0]);
                })
        });

        if uncolored_positions_next_to_colored.len() == 0 {
            // Can't keep going with this difficulty, try again with a harder difficulty
            flood_fill_color_map(color_map, board_size, difficulty + 0.1);
            return;
        }

        // Color them randomly
        uncolored_positions_next_to_colored.iter().for_each(|(pos, colors)| {
            let color = colors.choose(&mut rand::thread_rng()).unwrap();
            color_map[pos.1][pos.0] = *color;
            uncolored_positions.remove(pos);
        });
    }
}

#[derive(Debug, Clone)]
struct State {
    x_coords: Vec<Vec<usize>>,
    col_counts: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn it_generates_correct_amount_of_stars() {
        for i in 0..1000 {
            let star_positions = get_random_star_placements(2, 10);
            assert_eq!(star_positions.len(), 20, "Failed on iteration {}", i);
        }
    }

    #[test]
    fn it_generates_correct_board_size() {
        for i in 0..100 {
            let board = generate_board(2, 1.0);
            assert!(board.is_ok(), "Failed on iteration {}", i);
        }
    }
}
