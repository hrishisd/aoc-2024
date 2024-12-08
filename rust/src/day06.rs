use enumset::{EnumSet, EnumSetType};
use fixedbitset::FixedBitSet;

const INPUT: &str = include_str!("../../inputs/day06/input");
const N: u8 = aoc_rust::count_lines(INPUT) as u8;

fn main() {
    let mut obstacles = Obstacles::new();
    let mut initial_guard_pos: GridPos = GridPos { r: 0, c: 0 };
    for (row, line) in INPUT.lines().enumerate() {
        for (col, char) in line.bytes().enumerate() {
            let (row, col) = (row as u8, col as u8);
            match char {
                b'#' => {
                    obstacles.insert(GridPos { r: row, c: col });
                }
                b'^' => {
                    initial_guard_pos = GridPos { r: row, c: col };
                }
                _ => {}
            }
        }
    }
    let mut visited_part1 = [[EnumSet::empty(); N as usize]; N as usize];
    if let Route::PositionsVisited(n_positions) = walk(
        initial_guard_pos,
        Direction::Up,
        &obstacles,
        &mut visited_part1,
    ) {
        println!("part 1: {}", n_positions);
    } else {
        panic!("Got stuck while solving part 1");
    };

    let mut n_obstructions = 0;
    for row in 0..N {
        for col in 0..N {
            if visited_part1[row as usize][col as usize].is_empty() {
                continue;
            }
            let pos = GridPos { r: row, c: col };
            if obstacles.contains(pos) {
                continue;
            }
            // try putting an obstacle here
            obstacles.insert(pos);
            let mut visited = [[EnumSet::empty(); N as usize]; N as usize];
            if let Route::Stuck = walk(initial_guard_pos, Direction::Up, &obstacles, &mut visited) {
                n_obstructions += 1;
            }
            obstacles.remove(pos);
        }
    }
    println!("part 2: {}", n_obstructions);
}

enum Route {
    Stuck,
    PositionsVisited(usize),
}

fn walk(
    mut pos: GridPos,
    mut dir: Direction,
    obstacles: &Obstacles,
    visited: &mut [[EnumSet<Direction>; N as usize]; N as usize],
) -> Route {
    // let mut visited: [[EnumSet<Direction>; N as usize]; N as usize] =
    //     [[EnumSet::empty(); N as usize]; N as usize];
    while let Some(next_pos) = pos.step(dir) {
        if visited[next_pos.r as usize][next_pos.c as usize].contains(dir) {
            return Route::Stuck;
        }
        if obstacles.contains(next_pos) {
            dir = dir.turn_right();
        } else {
            visited[next_pos.r as usize][next_pos.c as usize].insert(dir);
            pos = next_pos;
        }
    }
    let n_cells_visited = visited
        .iter()
        .map(|inner| inner.iter().filter(|cell| !cell.is_empty()).count())
        .sum();
    Route::PositionsVisited(n_cells_visited)
}

struct Obstacles(FixedBitSet);
impl Obstacles {
    fn new() -> Self {
        Self(FixedBitSet::with_capacity(N as usize * N as usize))
    }
    fn to_idx(pos: GridPos) -> usize {
        pos.r as usize * N as usize + pos.c as usize
    }
    fn insert(&mut self, pos: GridPos) {
        self.0.insert(Obstacles::to_idx(pos));
    }
    fn contains(&self, pos: GridPos) -> bool {
        self.0.contains(Obstacles::to_idx(pos))
    }
    fn remove(&mut self, pos: GridPos) {
        self.0.remove(Obstacles::to_idx(pos));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPos {
    r: u8,
    c: u8,
}

impl GridPos {
    fn step(self, dir: Direction) -> Option<GridPos> {
        let (row, col) = match dir {
            Direction::Up => {
                if self.r == 0 {
                    return None;
                }
                (self.r - 1, self.c)
            }
            Direction::Down => {
                if self.r == N - 1 {
                    return None;
                }
                (self.r + 1, self.c)
            }
            Direction::Left => {
                if self.c == 0 {
                    return None;
                }
                (self.r, self.c - 1)
            }
            Direction::Right => {
                if self.c == N - 1 {
                    return None;
                }
                (self.r, self.c + 1)
            }
        };
        Some(GridPos { r: row, c: col })
    }
}

#[derive(Debug, Hash, EnumSetType)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Direction {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}
