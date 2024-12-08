use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day08/input");
const N: usize = aoc_rust::count_lines(INPUT);

fn main() {
    let mut groups: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for (row, line) in INPUT.lines().enumerate() {
        for (col, char) in line.bytes().enumerate() {
            if char != b'.' {
                groups.entry(char).or_default().push((row, col));
            }
        }
    }
    println!("N: {}", N);
    println!("part 1: {}", part1(&groups));
    println!("part 2: {}", part2(&groups));
}

fn part1(groups: &HashMap<u8, Vec<(usize, usize)>>) -> usize {
    let mut locations = HashSet::new();
    for group in groups.values() {
        for &(r, c) in group {
            for &(rr, cc) in group {
                if (r, c) == (rr, cc) {
                    continue;
                }
                let (r, c) = (r as i64, c as i64);
                let (rr, cc) = (rr as i64, cc as i64);
                let (dr, dc) = (rr - r, cc - c);
                let loc = (rr + dr, cc + dc);
                if in_grid_signed::<N>(loc) {
                    locations.insert(loc);
                }
            }
        }
    }
    locations.len()
}

fn part2(groups: &HashMap<u8, Vec<(usize, usize)>>) -> usize {
    let mut locations = HashSet::new();
    for group in groups.values() {
        for &(r, c) in group {
            for &(rr, cc) in group {
                if (r, c) == (rr, cc) {
                    continue;
                }
                let (r, c) = (r as i64, c as i64);
                let (rr, cc) = (rr as i64, cc as i64);
                let (dr, dc) = (rr - r, cc - c);
                let mut i = 0;
                while in_grid_signed::<N>(((rr + dr * i), (cc + dc * i))) {
                    let loc = ((rr + dr * i), (cc + dc * i));
                    locations.insert(loc);
                    i += 1
                }
            }
        }
    }
    locations.len()
}

const fn in_grid_signed<const N: usize>((row, col): (i64, i64)) -> bool {
    let n = N as i64;
    row >= 0 && col >= 0 && row < n && col < n
}
