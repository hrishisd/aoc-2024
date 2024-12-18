#![feature(const_for)]

use fixedbitset::FixedBitSet;
use std::array;

static INPUT: &str = include_str!("../../inputs/day10/input");
const N: usize = aoc_rust::count_lines(INPUT.as_bytes());
const GRID: [[u8; N]; N] = parse_grid();
const N_PEAKS: usize = n_peaks();

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn incline_neighbors(r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .filter_map(
            move |&(dr, dc)| match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                (Some(nr), Some(nc)) if nr < N && nc < N && GRID[nr][nc] == GRID[r][c] + 1 => {
                    Some((nr, nc))
                }
                _ => None,
            },
        )
}

fn part1() -> usize {
    let mut peaks_reachable: [[FixedBitSet; N]; N] =
        array::from_fn(|_| array::from_fn(|_| FixedBitSet::with_capacity(N_PEAKS)));
    let mut peak_idx = 0;
    for r in 0..N {
        for c in 0..N {
            if GRID[r][c] == 9 {
                peaks_reachable[r][c].insert(peak_idx);
                peak_idx += 1;
            }
        }
    }
    for elevation in (0..=8).rev() {
        for r in 0..N {
            for c in 0..N {
                if GRID[r][c] == elevation {
                    for (rr, cc) in incline_neighbors(r, c) {
                        peaks_reachable[r][c].union_with(&peaks_reachable[rr][cc].clone());
                    }
                }
            }
        }
    }

    GRID.iter()
        .flatten()
        .zip(peaks_reachable.iter().flatten())
        .filter(|&(&elevation, _)| elevation == 0)
        .map(|(_, peaks)| peaks.count_ones(..))
        .sum()
}

fn part2() -> usize {
    let mut n_trails_from = [[0; N]; N];
    for r in 0..N {
        for c in 0..N {
            if GRID[r][c] == 9 {
                n_trails_from[r][c] = 1;
            }
        }
    }
    for elevation in (0..=8).rev() {
        for r in 0..N {
            for c in 0..N {
                if GRID[r][c] == elevation {
                    n_trails_from[r][c] = incline_neighbors(r, c)
                        .map(|(rr, cc)| n_trails_from[rr][cc])
                        .sum()
                }
            }
        }
    }

    GRID.iter()
        .flatten()
        .zip(n_trails_from.iter().flatten())
        .map(|(&elevation, &trails)| if elevation == 0 { trails } else { 0 })
        .sum()
}

const fn parse_grid() -> [[u8; N]; N] {
    let mut res = [[0; N]; N];
    let mut idx = 0;
    let bytes = INPUT.as_bytes();
    let mut b_idx = 0;
    while b_idx < bytes.len() {
        let b = bytes[b_idx];
        if b != b'\n' {
            res[idx / N][idx % N] = if b == b'.' { u8::MAX } else { b - b'0' };
            idx += 1;
        }
        b_idx += 1;
    }
    res
}

const fn n_peaks() -> usize {
    let mut res = 0;
    let mut r = 0;
    while r < N {
        let mut c = 0;
        while c < N {
            if GRID[r][c] == 9 {
                res += 1;
            }
            c += 1;
        }
        r += 1;
    }
    res
}
