use fixedbitset::FixedBitSet;
use sscanf::scanf;
use std::{array, collections::HashSet};

const N_COMPUTERS: usize = 26 * 26;
type Computer = u16;

fn main() {
    let input = include_str!("../../inputs/day23/input");
    let connections: Vec<(Computer, Computer)> = input
        .lines()
        .map(|line| {
            let (l, r) = scanf!(line, "{str}-{str}").unwrap();
            (parse_computer(l), parse_computer(r))
        })
        .collect();

    let mut adj: [FixedBitSet; N_COMPUTERS] =
        array::from_fn(|_| FixedBitSet::with_capacity(N_COMPUTERS));

    for &(a, b) in &connections {
        adj[a as usize].insert(b as usize);
        adj[b as usize].insert(a as usize);
    }

    let mut groups = HashSet::new();
    for &(a, b) in &connections {
        for common in adj[a as usize].intersection(&adj[b as usize]) {
            let common = common as u16;
            if common == a || common == b {
                continue;
            }
            let mut group = [a, b, common];
            group.sort();
            groups.insert(group);
        }
    }

    println!(
        "part 1: {}",
        groups
            .iter()
            .filter(|group| group.iter().any(|&c| starts_with_t(c)))
            .count()
    );

    let mut largest_group = groups
        .iter()
        .map(|group| {
            let mut clique = vec![];
            clique.extend_from_slice(group);
            for c in 0..N_COMPUTERS {
                let c = c as u16;
                if clique.contains(&c) {
                    continue;
                }
                if clique
                    .iter()
                    .all(|&existing| adj[c as usize].contains(existing as usize))
                {
                    clique.push(c);
                }
            }
            clique
        })
        .max_by_key(|clique| clique.len())
        .unwrap()
        .iter()
        .map(|&c| computer_to_string(c))
        .collect::<Vec<_>>();
    largest_group.sort();
    println!("part 2: {}", largest_group.join(","));
}

fn computer_to_string(c: Computer) -> String {
    let (first, second) = (c / 26, c % 26);
    format!(
        "{}{}",
        (first as u8 + b'a') as char,
        (second as u8 + b'a') as char
    )
}

fn starts_with_t(c: Computer) -> bool {
    (c / 26) as u8 == b't' - b'a'
}

fn parse_computer(s: &str) -> Computer {
    if let &[first, second] = s.as_bytes() {
        ((first - b'a') as u16) * 26 + (second - b'a') as u16
    } else {
        panic!("bad computer: {s}");
    }
}
