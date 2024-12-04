fn main() {
    let input = include_str!("../../inputs/day04/input");
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    println!("part 1: {}", part1(&grid));
    println!("part 2: {}", part2(&grid));
}

fn part1(grid: &[&[u8]]) -> i32 {
    let has_xmas = |row: i32, col: i32, dr: i32, dc: i32| -> bool {
        let in_range = |r: i32, c: i32| -> bool {
            let n = grid.len() as i32;
            0 <= r && r < n && 0 <= c && c < n
        };
        let xmas = b"XMAS";
        xmas.iter().enumerate().all(|(i, char)| {
            let row = row + (i as i32 * dr);
            let col = col + (i as i32 * dc);
            in_range(row, col) && grid[row as usize][col as usize] == *char
        })
    };
    let mut result = 0;
    for r in 0..grid.len() {
        for c in 0..grid.len() {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if has_xmas(r as i32, c as i32, dr, dc) {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn part2(grid: &[&[u8]]) -> u32 {
    let mut result = 0;
    for r in 1..(grid.len() - 1) {
        for c in 1..(grid.len() - 1) {
            let major = [grid[r - 1][c - 1], grid[r][c], grid[r + 1][c + 1]];
            let minor = [grid[r + 1][c - 1], grid[r][c], grid[r - 1][c + 1]];
            if (&major == b"SAM" || &major == b"MAS") && (&minor == b"SAM" || &minor == b"MAS") {
                result += 1;
            }
        }
    }
    return result;
}
