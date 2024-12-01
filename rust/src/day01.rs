use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day01/input");
    let (mut left_col, mut right_col): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .unzip();
    left_col.sort_unstable();
    right_col.sort_unstable();

    println!("part 1: {}", part1(&left_col, &right_col));
    println!("part 2: {}", part2(&left_col, &right_col));
}

fn part1(left_col: &[u32], right_col: &[u32]) -> u32 {
    left_col
        .iter()
        .zip(right_col)
        .map(|(&l, &r)| l.abs_diff(r))
        .sum()
}

fn part2(left_col: &[u32], right_col: &[u32]) -> u32 {
    let counts = right_col.iter().fold(HashMap::new(), |mut counts, &elem| {
        *counts.entry(elem).or_insert(0) += 1;
        counts
    });
    left_col
        .iter()
        .map(|elem| elem * counts.get(elem).unwrap_or(&0))
        .sum()
}
