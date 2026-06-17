use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Runs A* on a grid of points.
///
/// Any generic `<T>` can be stored in the board.
///
/// The end condition is not always a single point, it can be a group, so it is generalized to just a
/// predicate on the position and value that is checked on every position.
///
/// TODO: Maybe make a more general graph version of this if needed
///
/// # Examples
/// ```
/// use utilities::astar::run_board_a_star;
/// // In this example, 0s are empty, 1s are walls, and 2s are the start and end.
/// // It is going from (0, 0) to anywhere on the bottom row.
/// // There is no heuristic.
/// let board = vec![vec![2, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 0, 0], vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 1, 1, 1], vec![2, 2, 2, 2, 2, 2]];
/// let start = (0, 0);
/// let end_condition = |pos, value| pos != start && value == 2;
/// let heuristic = |pos, value| 0;
/// let valid_space = |pos, value| value != 1;
/// let path = run_board_a_star(&board, start, end_condition, heuristic, valid_space);
/// assert_eq!(path.unwrap(), vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (4, 1), (4, 2), (3, 2), (2, 2), (2, 3), (2, 4)]);
/// ```
pub fn run_board_a_star<T: PartialEq + Eq + Copy>(board: &Vec<Vec<T>>, start: (usize, usize),
                                                  end_condition: impl Fn((usize, usize), T) -> bool,
                                                  heuristic: impl Fn((usize, usize), T) -> usize,
                                                  valid_space: impl Fn((usize, usize), T) -> bool) -> Option<Vec<(usize, usize)>> {
    let mut nodes_to_check = BinaryHeap::new();
    let mut g_scores: HashMap<(usize, usize), usize> = HashMap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let start_node = Node {
        position: start,
        f: heuristic(start, board[start.1][start.0])
    };
    nodes_to_check.push(start_node);
    g_scores.insert(start, 0);

    while let Some(node) = nodes_to_check.pop() {
        if end_condition(node.position, board[node.position.1][node.position.0]) {
            return Some(reconstruct_path(node, came_from));
        }

        for pos in neighbors(node.position, board[0].len(), board.len()) {
            if !valid_space(pos, board[pos.1][pos.0]) {
                continue;
            }

            let possible_score = g_scores[&node.position] + 1;

            if possible_score < *g_scores.entry(pos).or_insert(usize::MAX) {
                g_scores.insert(pos, possible_score);
                came_from.insert(pos, node.position);

                let new_node = Node {
                    position: pos,
                    f: possible_score + heuristic(pos, board[pos.1][pos.0])
                };

                nodes_to_check.push(new_node);
            }
        }
    }

    None
}

fn reconstruct_path(n: Node, came_from: HashMap<(usize, usize), (usize, usize)>) -> Vec<(usize, usize)> {
    let mut current_pos = &n.position;
    let mut path: Vec<(usize, usize)> = vec![*current_pos];

    while let Some(pos) = came_from.get(current_pos) {
        path.push(*pos);
        current_pos = pos;
    }

    path.reverse();
    path
}

fn neighbors(pos: (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    vec![(pos.0, pos.1 + 1),
         (pos.0, if pos.1 > 0 { pos.1 - 1 } else { max_y }),
         (pos.0 + 1, pos.1),
         (if pos.0 > 0 { pos.0 - 1 } else { max_x }, pos.1)]
        .into_iter()
        .filter(|pos| {
            pos.0 < max_x && pos.1 < max_y
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    position: (usize, usize),
    f: usize,
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.f).partial_cmp(&(other.f))?.reverse())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.f).cmp(&(other.f)).reverse()
    }
}