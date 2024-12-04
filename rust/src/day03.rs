use regex::{self, Regex};

fn main() {
    let input = include_str!("../../inputs/day03/input");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_sum, [lhs, rhs]) = c.extract();
            let lhs = lhs.parse::<u32>().unwrap();
            let rhs = rhs.parse::<u32>().unwrap();
            lhs * rhs
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    // group 1 -> do
    // group 2 -> don't
    // group 3 -> mul
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d+),(\d+)\))").unwrap();
    let mut res = 0;
    let mut mul_enabled = true;
    for m in re.captures_iter(input) {
        if let Some(_do) = m.get(1) {
            mul_enabled = true;
        } else if let Some(_dont) = m.get(2) {
            mul_enabled = false;
        } else if mul_enabled {
            if let Some(_mul) = m.get(3) {
                let lhs = m.get(4).unwrap().as_str().parse::<u32>().unwrap();
                let rhs = m.get(5).unwrap().as_str().parse::<u32>().unwrap();
                res += lhs * rhs;
            }
        }
    }
    res
}

#[test]
fn test_part_1() {
    let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(161, part1(example));
}

#[test]
fn test_part2() {
    let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(48, part2(example));
}
