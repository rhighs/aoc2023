use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Cities = Vec<Vec<usize>>;

fn parse_input(input: &String) -> Cities {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec()
}

#[derive(Hash, Ord, Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn is_opposite(&self, other: Dir) -> bool {
        use Dir::*;
        match *self {
            Up => other == Down,
            Down => other == Up,
            Right => other == Left,
            Left => other == Right,
        }
    }

    fn as_offset(&self) -> (i32, i32) {
        use Dir::*;
        match *self {
            Up => (-1, 0),
            Down => (1, 0),
            Right => (0, 1),
            Left => (0, -1),
        }
    }
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
struct HEntry((usize, Node, (Dir, usize)));
#[derive(Debug, Hash, PartialEq, Eq)]
struct VKey((Node, (Dir, usize)));
type Node = (usize, usize);

fn custom_dijkstra(
    cities: &Cities,
    dir_skip_condition: fn(Dir, Dir, usize) -> bool,
) -> Vec<(Node, usize, usize)> {
    // These complex keys allow for running dijstra on multiple paths basically
    // in this sense it's not really dijkstra anymore, but rather a BFS based
    // on local minimums. This helps us accounts for multiple paths later on.
    let mut h = BinaryHeap::<Reverse<HEntry>>::new();
    let mut v = HashMap::<VKey, usize>::new();

    // using a min heap allows for easy access to the next node we want to process
    h.push(Reverse(HEntry((0, (0, 0), (Dir::Down, 0)))));
    h.push(Reverse(HEntry((0, (0, 0), (Dir::Right, 0)))));

    let dirs = vec![Dir::Up, Dir::Right, Dir::Down, Dir::Left];
    while h.len() > 0 {
        if let Some(Reverse(HEntry((current_distance, (i, j), (node_dir, node_dir_steps))))) =
            h.pop()
        {
            for dir in &dirs {
                let (dir_i, dir_j) = dir.as_offset();
                let (i, j) = (i as i32, j as i32);

                // check bounds and direction
                {
                    if i + dir_i < 0
                        || i + dir_i >= cities.len() as i32
                        || j + dir_j < 0
                        || j + dir_j >= cities[0].len() as i32
                    {
                        continue;
                    } else if node_dir.is_opposite(*dir) {
                        continue;
                    }

                    // custom dir checks based on problem statement
                    if dir_skip_condition(node_dir, *dir, node_dir_steps) {
                        continue;
                    }
                }

                let node_adj = ((i + dir_i) as usize, (j + dir_j) as usize);
                let new_distance = current_distance + cities[node_adj.0][node_adj.1];
                let adj_node_dir_steps = if node_dir == *dir { node_dir_steps + 1 } else { 1 };

                // check if it was reached already by some other path, default to usize::MAX otherwise
                let key = VKey((node_adj, (*dir, adj_node_dir_steps)));
                let current_adj_distance = if let Some(entry) = v.get(&key) {
                    *entry
                } else {
                    usize::MAX
                };

                // reassign best path
                if new_distance < current_adj_distance {
                    v.insert(key, new_distance);
                    h.push(Reverse(HEntry((
                        new_distance,
                        node_adj,
                        (*dir, adj_node_dir_steps),
                    ))));
                }
            }
        }
    }

    // Collect entries that lead to the final node,
    // each one with different amont amount of steps, direction and path.
    // Part 1 and 2 need to process them differently.
    // (e.g part 2 must have the final node having at least 4 steps taken in the same direction)
    v.into_iter()
        .filter(|(VKey((node, ..)), ..)| {
            node.0 == cities.len() - 1 && node.1 == cities[0].len() - 1
        })
        .map(|(VKey((node, .., (_, steps_done))), cost)| (node, cost, steps_done))
        .collect_vec()
}

pub fn p1(input: &String) -> String {
    const MAX_DIR_STEPS: usize = 3;
    let cities = parse_input(input);
    let result = custom_dijkstra(&cities, |dir, new_dir, steps| {
        dir == new_dir && steps == MAX_DIR_STEPS
    });
    let (_, distance, _) = result
        .iter()
        .min_by(|(_, cost_a, ..), (_, cost_b, ..)| cost_a.cmp(cost_b))
        .unwrap();
    distance.to_string()
}

pub fn p2(input: &String) -> String {
    const MIN_DIR_STEPS: usize = 4;
    const MAX_DIR_STEPS: usize = 10;
    let cities = parse_input(input);
    let result = custom_dijkstra(&cities, |dir, new_dir, steps| {
        (dir == new_dir && steps == MAX_DIR_STEPS) || (dir != new_dir && steps < MIN_DIR_STEPS)
    });
    let (_, distance, _) = result
        .iter()
        .filter(|(.., steps_done)| *steps_done >= 4)
        .min_by(|(_, cost_a, ..), (_, cost_b, ..)| cost_a.cmp(cost_b))
        .unwrap();
    distance.to_string()
}
