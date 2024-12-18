#![feature(let_chains)]
use std::num::NonZeroUsize;

const INPUT: [u64; 8] = [7725, 185, 2, 132869, 0, 1840437, 62, 26310];

fn main() {
    println!("part 1: {}", solve(25));
    println!("part 1: {}", solve(75));
}

const fn solve(n_iter: usize) -> usize {
    let mut cache = [[None; 76]; 256];
    let mut i = 0;
    let mut res = 0;
    while i < INPUT.len() {
        res += count_stones(INPUT[i], n_iter, &mut cache);
        i += 1;
    }
    res
}

const fn count_stones(
    stone: u64,
    n_iter: usize,
    cache: &mut [[Option<NonZeroUsize>; 76]; 256],
) -> usize {
    if n_iter == 0 {
        return 1;
    }
    if stone < 256
        && let Some(val) = cache[stone as usize][n_iter]
    {
        return val.get();
    }
    let res = if stone == 0 {
        count_stones(1, n_iter - 1, cache)
    } else if let Some((l, r)) = split_digits(stone) {
        count_stones(l, n_iter - 1, cache) + count_stones(r, n_iter - 1, cache)
    } else {
        count_stones(stone * 2024, n_iter - 1, cache)
    };
    if stone < 100 {
        cache[stone as usize][n_iter] = NonZeroUsize::new(res);
    }
    res
}

const fn split_digits(stone: u64) -> Option<(u64, u64)> {
    assert!(stone != 0);
    let mut n_digits = 0u32;
    let mut temp = stone;
    while temp > 0 {
        temp /= 10;
        n_digits += 1;
    }
    if n_digits % 2 == 1 {
        None
    } else {
        let mask = 10u64.pow(n_digits / 2);
        let l = stone / mask;
        let r = stone % mask;
        Some((l, r))
    }
}
