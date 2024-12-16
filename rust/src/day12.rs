#![feature(array_windows)]
use fixedbitset::FixedBitSet;

// const INPUT: &[u8; 109] = include_bytes!("../../inputs/day12/example");
const INPUT: &[u8; 19739] = include_bytes!("../../inputs/day12/input");
const N: usize = aoc_rust::count_lines(INPUT);

fn main() {
    let mut grid = [[0u8; N]; N];
    for (idx, row) in INPUT.splitn(N, |&b| b == b'\n').enumerate() {
        grid[idx].copy_from_slice(row);
    }
    let mut visited = FixedBitSet::with_capacity(N * N);
    let mut part1 = 0;
    let mut part2 = 0;
    for r in 0..N {
        for c in 0..N {
            if !visited.contains(r * N + c) {
                let region = explore(&grid, r, c, &mut visited);
                part1 += region.len() * perimeter(&region, &grid);
                part2 += region.len() * sides(&region, &grid);
            }
        }
    }
    println!("part 1: {part1}");
    println!("part 2: {part2}");
}

fn explore(grid: &[[u8; N]; N], r: usize, c: usize, visited: &mut FixedBitSet) -> Vec<(u8, u8)> {
    fn explore_rec(
        grid: &[[u8; N]; N],
        r: usize,
        c: usize,
        acc: &mut Vec<(u8, u8)>,
        visited: &mut FixedBitSet,
    ) {
        if visited.contains(r * N + c) {
            return;
        }
        if acc.is_empty()
            || grid[r][c] == {
                let (rr, cc) = acc[0];
                grid[rr as usize][cc as usize]
            }
        {
            visited.insert(r * N + c);
            acc.push((r as u8, c as u8));
            let r = r as i32;
            let c = c as i32;
            for (rr, cc) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                let valid_range = 0..(N as i32);
                if valid_range.contains(&rr) && valid_range.contains(&cc) {
                    explore_rec(grid, rr as usize, cc as usize, acc, visited);
                }
            }
        }
    }
    let mut region = Vec::new();
    explore_rec(grid, r, c, &mut region, visited);
    region
}

fn perimeter(region: &[(u8, u8)], grid: &[[u8; N]; N]) -> usize {
    assert!(!region.is_empty());
    let kind = {
        let (r, c) = region[0];
        grid[r as usize][c as usize]
    };
    region
        .iter()
        .flat_map(|&(r, c)| {
            let r = r as i16;
            let c = c as i16;
            [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
        })
        .filter(|&(r, c)| {
            r < 0 || r >= N as i16 || c < 0 || c >= N as i16 || grid[r as usize][c as usize] != kind
        })
        .count()
}

fn sides(region: &[(u8, u8)], grid: &[[u8; N]; N]) -> usize {
    assert!(!region.is_empty());
    let kind = {
        let (r, c) = region[0];
        grid[r as usize][c as usize]
    };
    // return true if grid[r][c] is in range and == kind, else false
    let is_region = |(r, c): (i16, i16)| {
        0 <= r
            && (r as usize) < N
            && 0 <= c
            && (c as usize) < N
            && grid[r as usize][c as usize] == kind
    };
    let mut n_corners = 0;
    for &(r, c) in region {
        let r = r as i16;
        let c = c as i16;
        for [(dr1, dc1), (dr2, dc2)] in [
            (-1, 0), // up
            (0, 1),  // right
            (1, 0),  // down
            (0, -1), // left
            (-1, 0), // up
        ]
        .array_windows::<2>()
        {
            let side_1 = (r + dr1, c + dc1);
            let side_2 = (r + dr2, c + dc2);
            let corner = (r + dr1 + dr2, c + dc1 + dc2);
            n_corners += match (is_region(side_1), is_region(side_2)) {
                // interior corner
                (true, true) if !is_region(corner) => 1,
                // exterior corner
                (false, false) => 1,
                (true, false) | (false, true) | (true, true) => 0,
            }
        }
    }
    n_corners
}
