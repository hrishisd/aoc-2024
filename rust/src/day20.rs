#![feature(array_windows)]
use fixedbitset::FixedBitSet;
use std::collections::VecDeque;

static INPUT: &[u8] = include_bytes!("../../inputs/day20/input");
const N: usize = aoc_rust::count_lines(INPUT);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    r: usize,
    c: usize,
}

fn main() {
    let mut track_locs = FixedBitSet::with_capacity(N * N);
    let mut end = Pos { r: 0, c: 0 };
    for (r, row) in INPUT.splitn(N, |&b| b == b'\n').enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            match cell {
                b'.' | b'S' => track_locs.insert(Pos { r, c }.idx()),
                b'E' => end = Pos { r, c },
                _ => {}
            }
        }
    }

    let mut dist = [[u16::MAX; N]; N];
    let mut frontier = VecDeque::from([end]);
    let mut d = 0;
    while !frontier.is_empty() {
        for p in &frontier {
            dist[p.r][p.c] = d;
        }
        let new_frontier = frontier
            .iter()
            .flat_map(|pos| pos.n_steps(1))
            .filter(|&p| track_locs.contains(p.idx()) && dist[p.r][p.c] > d + 1)
            .collect();
        frontier = new_frontier;
        d += 1;
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for (r, row) in dist.iter().enumerate() {
        for (c, &d) in row.iter().enumerate() {
            if d != u16::MAX {
                for steps in 1..=20 {
                    for p in (Pos { r, c }.n_steps(steps as isize)) {
                        if improvement(d, dist[p.r][p.c], steps) >= 100 {
                            part2 += 1;
                            if steps == 2 {
                                part1 += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{part1}");
    println!("{part2}");
}

const fn improvement(old_dist: u16, new_dist: u16, step: u16) -> u16 {
    match old_dist.checked_sub(step) {
        Some(d) if d > new_dist => d - new_dist,
        _ => 0,
    }
}

impl Pos {
    const fn idx(self) -> usize {
        self.r * N + self.c
    }

    /// Returns an iterator of all positions reachable by taking exactly N steps from the start position
    fn n_steps(self, steps: isize) -> impl Iterator<Item = Self> {
        // up, right, down, left, up
        [(1isize, 0isize), (0, 1), (-1, 0), (0, -1), (1, 0)]
            .array_windows::<2>()
            .flat_map(move |&[(dr1, dc1), (dr2, dc2)]| {
                (1..=steps).filter_map(move |i| {
                    let j = steps - i;
                    match (
                        self.r.checked_add_signed(i * dr1 + j * dr2),
                        self.c.checked_add_signed(i * dc1 + j * dc2),
                    ) {
                        (Some(r), Some(c)) if r < N && c < N => Some(Self { r, c }),
                        _ => None,
                    }
                })
            })
    }
}

#[test]
fn test_steps() {
    use std::collections::HashSet;

    let p = Pos { r: 1, c: 1 };
    let reachable = p.n_steps(1).collect::<HashSet<_>>();
    assert_eq!(
        HashSet::from([
            Pos { r: 0, c: 1 },
            Pos { r: 1, c: 0 },
            Pos { r: 2, c: 1 },
            Pos { r: 1, c: 2 }
        ]),
        reachable
    );

    let reachable = p.n_steps(2).collect::<HashSet<_>>();
    assert_eq!(
        HashSet::from([
            Pos { r: 2, c: 0 },
            Pos { r: 3, c: 1 },
            Pos { r: 2, c: 2 },
            Pos { r: 1, c: 3 },
            Pos { r: 0, c: 2 },
            Pos { r: 0, c: 0 }
        ]),
        reachable
    );
}
