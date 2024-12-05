use fixedbitset::FixedBitSet;
use std::array;

/// The output is a mapping of elements to a sort key
fn get_topological_order(successors: &[FixedBitSet; 100]) -> [u8; 100] {
    let mut in_degrees = [0u8; 100];
    for succ in successors {
        for elem in succ.ones() {
            in_degrees[elem] += 1;
        }
    }
    let mut roots: Vec<u8> = in_degrees
        .iter()
        .enumerate()
        .filter(|(_, count)| **count == 0)
        .map(|(elem, _)| elem as u8)
        .collect();
    let mut ordering = [0; 100];
    let mut ordering_next_idx = 0;
    while let Some(root) = roots.pop() {
        ordering[ordering_next_idx] = root;
        ordering_next_idx += 1;
        for other in successors[root as usize].ones() {
            let d = &mut in_degrees[other];
            *d -= 1;
            if *d == 0 {
                roots.push(other as u8);
            }
        }
    }
    if ordering_next_idx < 100 {
        for idx in 0..100 {
            let d = in_degrees[idx];
            if d > 0 {
                println!("{idx} -> {d}")
            }
        }
        println!("in_degrees: {:?}", in_degrees);
    }
    assert_eq!(
        100, ordering_next_idx,
        "Ended without pushing exaectly 100 elements into order"
    );
    // println!("ordering:\n{:?}", ordering);
    let mut mapping = [0; 100];
    for (idx, &elem) in ordering.iter().enumerate() {
        mapping[elem as usize] = idx as u8;
    }

    mapping
}

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
    let ordering = get_topological_order(&successors);
    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .partition(|update| update.is_sorted_by_key(|&elem| ordering[elem as usize]));

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
