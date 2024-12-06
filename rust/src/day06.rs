use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day06/example");
const N: u8 = count_lines(INPUT) as u8;

fn main() {
    let mut obstacles = HashSet::new();
    let mut initial_guard_pos: GridPos = GridPos { row: 0, col: 0 };
    for (row, line) in INPUT.lines().enumerate() {
        for (col, char) in line.bytes().enumerate() {
            let (row, col) = (row as u8, col as u8);
            match char {
                b'#' => {
                    obstacles.insert(GridPos { row, col });
                }
                b'^' => {
                    initial_guard_pos = GridPos { row, col };
                }
                _ => {}
            }
        }
    }
    if let Route::PositionsVisited(n_positions) = walk(initial_guard_pos, Direction::UP, &obstacles)
    {
        println!("part 1: {}", n_positions);
    } else {
        panic!("Got stuck while solving part 1");
    };

    let mut n_obstructions = 0;
    for row in 0..N {
        for col in 0..N {
            let pos = GridPos { row, col };
            if obstacles.contains(&pos) {
                continue;
            }
            // try putting an obstacle here
            obstacles.insert(pos);
            if let Route::Stuck = walk(initial_guard_pos, Direction::UP, &obstacles) {
                n_obstructions += 1;
            }
            obstacles.remove(&pos);
        }
    }
    println!("part 2: {}", n_obstructions);
}

enum Route {
    Stuck,
    PositionsVisited(usize),
}

fn walk(mut pos: GridPos, mut dir: Direction, obstacles: &HashSet<GridPos>) -> Route {
    let mut visited: HashSet<(GridPos, Direction)> = HashSet::new();
    visited.insert((pos, dir));
    while let Some(next_pos) = pos.step(dir) {
        if visited.contains(&(next_pos, dir)) {
            return Route::Stuck;
        }
        if obstacles.contains(&next_pos) {
            dir = dir.turn_right();
        } else {
            visited.insert((next_pos, dir));
            pos = next_pos;
        }
    }
    Route::PositionsVisited(
        visited
            .iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>()
            .len(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPos {
    row: u8,
    col: u8,
}

impl GridPos {
    fn step(self, dir: Direction) -> Option<GridPos> {
        let (row, col) = match dir {
            Direction::UP => {
                if self.row == 0 {
                    return None;
                }
                (self.row - 1, self.col)
            }
            Direction::DOWN => {
                if self.row == N - 1 {
                    return None;
                }
                (self.row + 1, self.col)
            }
            Direction::LEFT => {
                if self.col == 0 {
                    return None;
                }
                (self.row, self.col - 1)
            }
            Direction::RIGHT => {
                if self.col == N - 1 {
                    return None;
                }
                (self.row, self.col + 1)
            }
        };
        Some(GridPos { row, col })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn turn_right(self) -> Direction {
        use Direction::*;
        match self {
            UP => RIGHT,
            DOWN => LEFT,
            LEFT => UP,
            RIGHT => DOWN,
        }
    }
}

const fn count_lines(s: &str) -> usize {
    let mut count = 1;
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'\n' {
            count += 1;
        }
        i += 1;
    }

    // If the string ends with a newline, don't count an extra line
    if bytes.len() > 0 && bytes[bytes.len() - 1] == b'\n' {
        count -= 1;
    }

    count
}
