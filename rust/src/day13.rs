use sscanf::scanf;
const INPUT: &str = include_str!("../../inputs/day13/input");

#[derive(Debug, Clone, Copy)]
/// a * x1 + b * x2 = x
/// a * y1 + b * y2 = y
struct Equations {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    x: i64,
    y: i64,
}

fn main() {
    let machines: Vec<Equations> = INPUT.split("\n\n").map(|block| parse_eq(block)).collect();
    let part1: i64 = machines
        .iter()
        .filter_map(|machine| solve(*machine))
        .map(|(a, b)| 3 * a + b)
        .sum();
    println!("part 1: {part1}");
    let part2: i64 = machines
        .iter()
        .map(|sys| Equations {
            x: sys.x + 10000000000000,
            y: sys.y + 10000000000000,
            ..*sys
        })
        .filter_map(|machine| solve(machine))
        .map(|(a, b)| 3 * a + b)
        .sum();
    println!("part 2: {part2}");
}

fn solve(system: Equations) -> Option<(i64, i64)> {
    let x = system.x as f64;
    let x1 = system.x1 as f64;
    let x2 = system.x2 as f64;
    let y = system.y as f64;
    let y1 = system.y1 as f64;
    let y2 = system.y2 as f64;
    let b = (x * y1 - x1 * y) / (x2 * y1 - x1 * y2);
    let delta = 0.01;
    let b_rounded = b.round();
    if (b - b_rounded).abs() > delta {
        return None;
    }
    let a = y / y1 - y2 / y1 * b_rounded;
    let a_rounded = a.round();
    if (a - a_rounded).abs() > delta {
        return None;
    }
    Some((a_rounded as i64, b_rounded as i64))
}

fn parse_eq(block: &str) -> Equations {
    let mut lines = block.lines();
    let (x1, y1) = scanf!(lines.next().unwrap(), "Button A: X+{i64}, Y+{i64}").unwrap();
    let (x2, y2) = scanf!(lines.next().unwrap(), "Button B: X+{i64}, Y+{i64}").unwrap();
    let (x, y) = scanf!(lines.next().unwrap(), "Prize: X={i64}, Y={i64}").unwrap();
    Equations {
        x1,
        y1,
        x2,
        y2,
        x,
        y,
    }
}
