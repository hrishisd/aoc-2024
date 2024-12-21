#![feature(ascii_char)]
use std::collections::{HashMap, HashSet, VecDeque};

// static INPUT: &[u8] = include_bytes!("../../inputs/day16/example");
static INPUT: &[u8] = include_bytes!("../../inputs/day16/input");
static N: usize = aoc_rust::count_lines(INPUT);

fn main() {
    let mut grid = [[0u8; N]; N];
    for (idx, row) in INPUT.splitn(N, |&b| b == b'\n').enumerate() {
        grid[idx].copy_from_slice(&row[..N]);
    }
    let (start_r, start_c) = (N - 2, 1);
    assert!(grid[start_r][start_c] == b'S');
    let goal = (1, N - 2);
    assert!(grid[goal.0][goal.1] == b'E');

    let mut queue = VecDeque::new();
    let start = State {
        r: start_r,
        c: start_c,
        dir: Direction::Right,
    };
    queue.push_back((start, start, 0));
    let mut min_cost = HashMap::new();
    let mut best_predececcors: HashMap<State, Vec<State>> = HashMap::new();
    while let Some((curr, prev, cost)) = queue.pop_front() {
        if grid[curr.r][curr.c] == b'#' {
            continue;
        }
        if let Some(&prev_cost) = min_cost.get(&curr) {
            if prev_cost < cost {
                continue;
            }
            if prev_cost == cost {
                best_predececcors.get_mut(&curr).unwrap().push(prev);
                continue;
            }
        }
        min_cost.insert(curr, cost);
        best_predececcors.insert(curr, vec![prev]);
        for dir in curr.dir.rotate() {
            let state = State { dir, ..curr };
            queue.push_back((state, curr, cost + 1000));
        }
        if let Some((r, c)) = curr.dir.step(curr.r, curr.c) {
            queue.push_back((
                State {
                    r,
                    c,
                    dir: curr.dir,
                },
                curr,
                cost + 1,
            ));
        }
    }

    let part1 = [
        Direction::Down,
        Direction::Left,
        Direction::Up,
        Direction::Right,
    ]
    .iter()
    .filter_map(|&dir| {
        min_cost.get(&State {
            r: goal.0,
            c: goal.1,
            dir,
        })
    })
    .min()
    .unwrap();
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited = HashSet::new();
    for dir in [
        Direction::Down,
        Direction::Left,
        Direction::Up,
        Direction::Right,
    ] {
        let state = State {
            r: goal.0,
            c: goal.1,
            dir,
        };
        if min_cost.get(&state).unwrap() == part1 {
            queue.push_back(state);
        }
    }
    while let Some(s @ State { r, c, .. }) = queue.pop_front() {
        visited.insert((r, c));
        for &pred in best_predececcors.get(&s).unwrap() {
            // don't get stuck in inf loop for start state
            if pred != s {
                queue.push_back(pred);
            }
        }
    }

    println!("part1: {part1}");
    println!("part2: {}", visited.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn step(self, r: usize, c: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up if r > 0 => Some((r - 1, c)),
            Direction::Down if r < N - 1 => Some((r + 1, c)),
            Direction::Left if c > 0 => Some((r, c - 1)),
            Direction::Right if c < N - 1 => Some((r, c + 1)),
            _ => None,
        }
    }

    /// returns the two possible directions with 90 degree rotations
    const fn rotate(self) -> [Self; 2] {
        use Direction::*;
        match self {
            Up | Down => [Left, Right],
            Left | Right => [Up, Down],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    r: usize,
    c: usize,
    dir: Direction,
}
