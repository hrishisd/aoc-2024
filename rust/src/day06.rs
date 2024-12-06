use enumset::{EnumSet, EnumSetType};
use fixedbitset::FixedBitSet;

const INPUT: &str = include_str!("../../inputs/day06/input");
const N: u8 = count_lines(INPUT) as u8;

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
    if let Route::PositionsVisited(n_positions) = walk(initial_guard_pos, Direction::UP, &obstacles)
    {
        println!("part 1: {}", n_positions);
    } else {
        panic!("Got stuck while solving part 1");
    };

    let mut n_obstructions = 0;
    for row in 0..N {
        for col in 0..N {
            let pos = GridPos { r: row, c: col };
            if obstacles.contains(pos) {
                continue;
            }
            // try putting an obstacle here
            obstacles.insert(pos);
            if let Route::Stuck = walk(initial_guard_pos, Direction::UP, &obstacles) {
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

fn walk(mut pos: GridPos, mut dir: Direction, obstacles: &Obstacles) -> Route {
    let mut visited: [[EnumSet<Direction>; N as usize]; N as usize] =
        [[EnumSet::empty(); N as usize]; N as usize];
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
            Direction::UP => {
                if self.r == 0 {
                    return None;
                }
                (self.r - 1, self.c)
            }
            Direction::DOWN => {
                if self.r == N - 1 {
                    return None;
                }
                (self.r + 1, self.c)
            }
            Direction::LEFT => {
                if self.c == 0 {
                    return None;
                }
                (self.r, self.c - 1)
            }
            Direction::RIGHT => {
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
