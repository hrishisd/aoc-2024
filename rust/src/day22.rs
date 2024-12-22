use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct Change {
    new_secret: u64,
    price: u8,
    delta: i8,
}

fn main() {
    let input = include_str!("../../inputs/day22/input");
    let input: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    println!(
        "part 1: {}",
        input.iter().map(|&i| step_n(i, 2000)).sum::<u64>()
    );

    let mut changes_to_profit = HashMap::new();
    for secret in input {
        count_profits(secret, &mut changes_to_profit);
    }
    println!(
        "part 2: {:?}",
        changes_to_profit.iter().max_by_key(|&(_k, v)| v).unwrap()
    );
}

fn count_profits(initial_secret: u64, profits: &mut HashMap<[i8; 4], u64>) {
    let mut seen: HashSet<[i8; 4]> = HashSet::new();
    let mut window = [i8::MAX; 4];
    let mut secret = initial_secret;
    for i in 0..2000 {
        let next = step(secret);
        window.rotate_left(1);
        window[3] = next.delta;
        secret = next.new_secret;
        if i >= 3 && !seen.contains(&window) {
            let count = profits.entry(window).or_insert(0);
            *count += next.price as u64;
            seen.insert(window);
        }
    }
}

fn step_n(mut secret: u64, n: usize) -> u64 {
    for _ in 0..n {
        secret = step(secret).new_secret;
    }
    secret
}

const fn step(secret: u64) -> Change {
    let n = prune(mix(secret, secret * 64));
    let n = prune(mix(n, n / 32));
    let new_secret = prune(mix(n, n * 2048));
    let price = (new_secret % 10) as u8;
    let prev_price = (secret % 10) as u8;
    let delta = price as i8 - prev_price as i8;
    Change {
        new_secret,
        price,
        delta,
    }
}

const fn prune(n: u64) -> u64 {
    n % 16777216
}

const fn mix(n: u64, other: u64) -> u64 {
    n ^ other
}
