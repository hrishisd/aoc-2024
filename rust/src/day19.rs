#![feature(assert_matches)]
use std::assert_matches::assert_matches;
use std::collections::HashSet;

fn main() {
    let mut lines = include_str!("../../inputs/day19/input").lines();
    let patterns: HashSet<&[u8]> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::as_bytes)
        .collect();
    assert_matches!(lines.next(), Some(""));
    let designs: Vec<&[u8]> = lines.map(str::as_bytes).collect();
    let max_pattern_len = patterns.iter().map(|p| p.len()).max().unwrap();

    let (part1, part2) = designs
        .iter()
        .map(|d| n_ways(d, &patterns, max_pattern_len))
        .fold((0, 0), |(possible, total), i| {
            (possible + (i > 0) as u64, total + i)
        });
    println!("part 1: {part1}\npart 2: {part2}");
}

fn n_ways(design: &[u8], patterns: &HashSet<&[u8]>, max_pattern_len: usize) -> u64 {
    let n = design.len();
    let mut cache = vec![0; n + 1];
    cache[n] = 1;
    for idx in (0..n).rev() {
        for pattern_len in 1..=std::cmp::min(max_pattern_len, n - idx) {
            if patterns.contains(&design[idx..(idx + pattern_len)]) {
                cache[idx] += cache[idx + pattern_len]
            }
        }
    }
    cache[0]
}

#[test]
fn test_n_ways() {
    let design = b"br";
    let patterns = HashSet::from([b"b", b"r", b"br"] as [&[u8]; 3]);
    assert_eq!(2, n_ways(design, &patterns, 2));
}
