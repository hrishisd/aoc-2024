#![feature(assert_matches)]

use sscanf::scanf;
use std::{
    assert_matches::assert_matches,
    collections::{HashSet, VecDeque},
};

static INPUT: &str = include_str!("../../inputs/day18/input");
const N: usize = 71;

fn parse_line(line: &str) -> (u8, u8) {
    scanf!(line, "{u8},{u8}").unwrap()
}

fn neighbors(r: u8, c: u8) -> impl Iterator<Item = (u8, u8)> {
    [(1i8, 0i8), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .filter_map(
            move |&(dr, dc)| match (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                (Some(rr), Some(cc)) if rr < N as u8 && cc < N as u8 => Some((rr, cc)),
                _ => None,
            },
        )
}

fn main() {
    let bytes: Vec<(u8, u8)> = INPUT.lines().map(parse_line).collect();
    println!("part1: {:?}", find_path(&bytes[..1024]));
    println!("part2: {:?}", find_first_blocking_byte(&bytes));
}

fn find_first_blocking_byte(bytes: &[(u8, u8)]) -> (u8, u8) {
    let mut l = 0;
    let mut r = bytes.len() - 1;
    while l < r {
        let mid = (l + r) / 2;
        match find_path(&bytes[..=mid]) {
            Some(_) => l = mid + 1,
            None => r = mid,
        }
    }
    assert_matches!(find_path(&bytes[..=l]), None);
    assert_matches!(find_path(&bytes[..l]), Some(_));
    bytes[l]
}

fn find_path(bytes: &[(u8, u8)]) -> Option<usize> {
    let blocked: HashSet<(u8, u8)> = HashSet::from_iter(bytes.iter().copied());
    // println!("blocked: {blocked:?}");
    let mut frontier = VecDeque::from([(0u8, 0u8)]);
    let mut visited = [[false; N]; N];
    visited[0][0] = true;
    for dist in 0..N * N {
        let mut next_frontier = VecDeque::new();
        for (r, c) in frontier {
            if (r, c) == (N as u8 - 1, N as u8 - 1) {
                return Some(dist);
            }
            assert!(!blocked.contains(&(r, c)));
            for (rr, cc) in neighbors(r, c) {
                if !blocked.contains(&(rr, cc)) && !visited[rr as usize][cc as usize] {
                    next_frontier.push_back((rr, cc));
                    visited[rr as usize][cc as usize] = true;
                }
            }
        }
        frontier = next_frontier;
    }
    None
}
