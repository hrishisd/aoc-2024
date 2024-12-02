#![feature(array_windows)]

fn main() {
    let input = include_str!("../../inputs/day02/input");
    let reports: Vec<Vec<i8>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i8>().unwrap())
                .collect()
        })
        .collect();

    println!(
        "part 1: {}",
        reports.iter().filter(|report| is_safe(report)).count()
    );

    println!(
        "part 1: {}",
        reports
            .iter()
            .filter(|report| is_safe_dampened(report))
            .count()
    );
}

fn is_safe(report: &[i8]) -> bool {
    (report.is_sorted() || report.iter().rev().is_sorted())
        && report.array_windows::<2>().all(|&[x, y]| {
            let diff = x.abs_diff(y);
            (1..=3).contains(&diff)
        })
}

fn is_safe_dampened(report: &[i8]) -> bool {
    if is_safe(report) {
        return true;
    }
    for remove_idx in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(remove_idx);
        if is_safe(&modified_report) {
            return true;
        }
    }
    false
}
