const INPUT: &str = include_str!("../../inputs/day07/input");

fn main() {
    let equations: Vec<(u64, Vec<u64>)> = INPUT
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            let rhs = rhs
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            (lhs.parse::<u64>().unwrap(), rhs)
        })
        .collect();
    let mut part1 = 0;
    let mut part2 = 0;
    for (lhs, rhs) in &equations {
        if can_make(*lhs, rhs, false) {
            part1 += lhs;
        }
        if can_make(*lhs, rhs, true) {
            part2 += lhs;
        }
    }
    println!("part 1: {part1}");
    println!("part 2: {part2}");
}

fn can_make(want: u64, nums: &[u64], use_concat: bool) -> bool {
    fn can_make_rec(want: u64, nums: &[u64], acc: u64, use_concat: bool) -> bool {
        if nums.is_empty() {
            acc == want
        } else {
            can_make_rec(want, &nums[1..], acc * nums[0], use_concat)
                || can_make_rec(want, &nums[1..], acc + nums[0], use_concat)
                || (use_concat
                    && can_make_rec(want, &nums[1..], concat_digits(acc, nums[0]), use_concat))
        }
    }
    can_make_rec(want, &nums[1..], nums[0], use_concat)
}

const fn concat_digits(l: u64, r: u64) -> u64 {
    let n_digits = r.ilog10() + 1;
    l * (10u64.pow(n_digits)) + r
}

#[test]
fn test_concat_digits() {
    assert_eq!(156, concat_digits(15, 6))
}
