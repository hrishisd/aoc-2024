use fixedbitset::FixedBitSet;
use std::{array, cmp::Ordering};

fn main() {
    let input = include_str!("../../inputs/day05/input");
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let successors: [FixedBitSet; 100] = rules
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("|").unwrap();
            (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap())
        })
        .fold(
            array::from_fn(|_| FixedBitSet::with_capacity(100)),
            |mut successors_acc, (l, r)| {
                successors_acc[l as usize].put(r as usize);
                successors_acc
            },
        );
    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .partition(|update| {
            update.is_sorted_by(|&x, &y| compare(x, y, &successors) == Ordering::Less)
        });

    println!(
        "part 1: {}",
        valid_updates
            .iter()
            .map(|xs| xs[xs.len() / 2] as u32)
            .sum::<u32>()
    );
    println!(
        "part 2: {}",
        invalid_updates
            .into_iter()
            .map(|mut update| {
                update.sort_by(|&x, &y| compare(x, y, &successors));
                update
            })
            .map(|xs| xs[xs.len() / 2] as u32)
            .sum::<u32>()
    );
}

fn compare(x: u8, y: u8, successors: &[FixedBitSet; 100]) -> Ordering {
    if successors[x as usize].contains(y as usize) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
