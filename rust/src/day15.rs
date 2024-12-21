#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Box,
    Wall,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    r: usize,
    c: usize,
}

fn main() {
    let input = include_str!("../../inputs/day15/input");
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();
    let grid_lines: Vec<&str> = grid_str.lines().collect();
    let (r, c) = grid_lines
        .iter()
        .enumerate()
        .filter_map(|(r, line)| {
            line.bytes()
                .enumerate()
                .find(|&(_, b)| b == b'@')
                .map(|(c, _)| (r, c))
        })
        .next()
        .unwrap();
    let robot = Pos { r, c };
    let mut grid: Vec<Vec<Cell>> = grid_lines
        .iter()
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'O' => Cell::Box,
                    b'#' => Cell::Wall,
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();

    let moves: Vec<Dir> = moves_str
        .bytes()
        .filter_map(|b| match b {
            b'^' => Some(Dir::Up),
            b'<' => Some(Dir::Left),
            b'>' => Some(Dir::Right),
            b'v' => Some(Dir::Down),
            _ => None,
        })
        .collect();

    let mut mut_grid_view = grid
        .iter_mut()
        .map(|row| row.as_mut_slice())
        .collect::<Vec<_>>();

    // println!("robot: {robot:?}");
    println!("{}", part1(robot, &moves, &mut mut_grid_view));
}

fn part1(mut robot: Pos, moves: &[Dir], grid: &mut [&mut [Cell]]) -> usize {
    // println!("start state:\n{:?}", robot);
    // print_grid(grid, robot);

    for &dir in moves {
        // println!("\nStep: {dir:?}");
        robot = step(robot, dir, grid);
        // print_grid(grid, robot);
    }
    let mut score = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == Cell::Box {
                score += 100 * r + c
            }
        }
    }
    score
}

fn step(pos: Pos, dir: Dir, grid: &mut [&mut [Cell]]) -> Pos {
    assert_eq!(Cell::Empty, grid[pos.r][pos.c]);
    let next = pos.step(dir);
    match grid[next.r][next.c] {
        Cell::Wall => pos,
        Cell::Empty => next,
        Cell::Box => match scan_for_space(next, dir, grid) {
            Some(space) => {
                grid[space.r][space.c] = Cell::Box;
                grid[next.r][next.c] = Cell::Empty;
                next
            }
            None => pos,
        },
    }
}

const fn scan_for_space(from_pos: Pos, dir: Dir, grid: &[&mut [Cell]]) -> Option<Pos> {
    let mut p = from_pos;
    loop {
        match grid[p.r][p.c] {
            Cell::Box => p = p.step(dir),
            Cell::Wall => return None,
            Cell::Empty => return Some(p),
        }
    }
}

impl Pos {
    const fn step(self, m: Dir) -> Self {
        use Dir::*;
        match m {
            Up => Self {
                r: self.r - 1,
                ..self
            },
            Down => Self {
                r: self.r + 1,
                ..self
            },
            Left => Self {
                c: self.c - 1,
                ..self
            },
            Right => Self {
                c: self.c + 1,
                ..self
            },
        }
    }
}
