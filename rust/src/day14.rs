use std::cmp;

use sscanf::scanf;

// static INPUT: &str = include_str!("../../inputs/day14/example");
static INPUT: &str = include_str!("../../inputs/day14/input");
const N_ROWS: usize = 103;
const N_COLS: usize = 101;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    r: i64,
    c: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Velocity {
    dr: i64,
    dc: i64,
}

fn main() {
    let robots: Vec<(Pos, Velocity)> = INPUT.lines().map(parse_robot).collect();
    println!("part 1: {}", part1(&robots, 100));
    part2(&robots);
}

fn part1(robots: &[(Pos, Velocity)], n_iter: i64) -> u32 {
    let (mut top_left, mut top_right, mut bottom_left, mut bottom_right) = (0, 0, 0, 0);
    let middle_col = N_COLS as i64 / 2;
    let middle_row = N_ROWS as i64 / 2;

    for &(p, v) in robots {
        let final_r = (p.r + (v.dr) * n_iter).rem_euclid(N_ROWS as i64);
        let final_c = (p.c + (v.dc) * n_iter).rem_euclid(N_COLS as i64);
        assert!(final_r >= 0);
        assert!(final_c >= 0);
        use cmp::Ordering::*;
        match (final_r.cmp(&middle_row), final_c.cmp(&middle_col)) {
            (Less, Less) => top_left += 1,
            (Less, Greater) => top_right += 1,
            (Greater, Less) => bottom_left += 1,
            (Greater, Greater) => bottom_right += 1,
            _ => { /* falls on the middle row or column */ }
        }
    }

    top_left * top_right * bottom_left * bottom_right
}

fn part2(robots: &[(Pos, Velocity)]) {
    for t in 0..1000 {
        let t = 55 + 101 * t;
        let mut positions = [[false; N_COLS]; N_ROWS];
        for &(p, v) in robots {
            let final_r = (p.r + (v.dr) * t).rem_euclid(N_ROWS as i64);
            let final_c = (p.c + (v.dc) * t).rem_euclid(N_COLS as i64);
            positions[final_r as usize][final_c as usize] = true;
        }
        println!("{}", "-".repeat(101));
        println!("\n{t}");
        for row in positions {
            let row: String = row.iter().map(|&r| if r { '*' } else { ' ' }).collect();
            println!("{row}");
        }
    }
}

fn parse_robot(line: &str) -> (Pos, Velocity) {
    let (c, r, dc, dr) = scanf!(line, "p={i64},{i64} v={i64},{i64}").unwrap();
    (Pos { r, c }, Velocity { dr, dc })
}
