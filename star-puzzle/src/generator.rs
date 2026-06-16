use std::collections::HashSet;
use itertools::Itertools;
use rand::prelude::{SliceRandom};
use crate::board::Board;

pub fn generate_board(max_star_count: usize) -> Result<Board, String> {
    validate_arguments(max_star_count)?;
    
    let board_size = max_star_count * 5;

    // 1. Randomly Place Stars
    let star_placements = get_random_star_placements(max_star_count, board_size);
    // 2. Seed color sections
        // 1. Make groups of `max_star_count` (maybe by: 1. picking a random star, 2. finding `max_star_count - 1` closest stars, repeat
        // 2. Find the easiest way to connect the groups (I am guessing, A* from 1 -> 2 then from 3 -> the 1 -> 2 path)
        // 3. If a path is not possible, restart from step 1 (maybe retry x times and if that doesn't work get a new random star placement?)
    // 3. Flood fill color sections
        // 1. Get a set of all uncolored positions
        // 2. While there are still uncolored positions:
            // 1. Get a list of all positions adjacent to some colored position
            // 2. For each one, give it the color of a random adjacent color
            // 3. Remove the colored position from the list of uncolored positions

    todo!()
}

fn validate_arguments(max_star_count: usize) -> Result<(), String> {
    if max_star_count > 5 {
        return Err("Star count must be at most 5".to_string())
    }

    Ok(())
}

fn get_random_star_placements(max_star_count: usize, board_size: usize) -> HashSet<(usize, usize)> {
    let initial_state = State {
        x_coords: vec![],
        col_counts: vec![0; board_size]
    };

    let solved_state = get_random_star_placements_backtrace(initial_state, max_star_count).expect("No solution found");

    let mut result: HashSet<(usize, usize)> = HashSet::new();

    solved_state.x_coords.iter().enumerate().for_each(|(y, x_coords)| {
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

    for choice in all_random_indices_from(&state.col_counts, max_star_count, state.x_coords.last().unwrap_or(&vec![])) {
        let mut next_state = State {
            x_coords: state.x_coords.clone(),
            col_counts: state.col_counts.clone()
        };

        for i in choice.iter() {
            next_state.col_counts[*i] += 1;
        }
        next_state.x_coords.push(choice);

        match get_random_star_placements_backtrace(next_state, max_star_count) {
            Some(s) => return Some(s),
            None => ()
        };
    }

    None
}

#[derive(Debug, Clone)]
struct State {
    x_coords: Vec<Vec<usize>>,
    col_counts: Vec<usize>
}

fn all_random_indices_from(col_counts: &Vec<usize>, max_star_count: usize, not_allowed: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = col_counts.iter()
        .enumerate()
        .filter(|(_, count)| **count < max_star_count)
        .map(|(i, _)| i)
        .filter(|i| !not_allowed.contains(i))
        .combinations(max_star_count)
        .collect();

    result.shuffle(&mut rand::thread_rng());

    result
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
}