use fixedbitset::FixedBitSet;
use std::array;

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
        .partition(|update| is_valid_update(&successors, update));

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
            .iter()
            .map(|invalid| make_valid_update(invalid, &successors))
            .map(|xs| xs[xs.len() / 2] as u32)
            .sum::<u32>()
    );
}

fn make_valid_update(update: &[u8], successors: &[FixedBitSet; 100]) -> Vec<u8> {
    let mut in_degrees = [0u8; 100];
    for &elem in update {
        for &other in update {
            if successors[other as usize].contains(elem as usize) {
                in_degrees[elem as usize] += 1;
            }
        }
    }
    let mut roots: Vec<u8> = update
        .iter()
        .copied()
        .filter(|&x| in_degrees[x as usize] == 0)
        .collect();
    let mut order = Vec::with_capacity(update.len());
    while let Some(root) = roots.pop() {
        order.push(root);
        for &elem in update {
            if successors[root as usize].contains(elem as usize) {
                let degree = &mut in_degrees[elem as usize];
                *degree -= 1;
                if *degree == 0 {
                    roots.push(elem);
                }
            }
        }
    }
    order
}

fn is_valid_update(successors: &[FixedBitSet; 100], update: &[u8]) -> bool {
    for (idx, &elem) in update.iter().enumerate() {
        // if elem is successor of anything after it, update is invalid
        if update[(idx + 1)..]
            .iter()
            .any(|&after| successors[after as usize].contains(elem as usize))
        {
            return false;
        }
    }
    true
}
