#![feature(iter_intersperse)]
use std::collections::VecDeque;
const PROGRAM: [u8; 16] = [2, 4, 1, 1, 7, 5, 0, 3, 4, 3, 1, 6, 5, 5, 3, 0];

fn main() {
    let initial_state = State {
        a: 18427963,
        b: 0,
        c: 0,
        ip: 0,
    };
    println!("{}", part1(initial_state));
    println!("{}", part2());
}

const fn execute_one_pass(a: u64) -> u8 {
    let mut b = a % 8;
    b ^= 0b001;
    let c = a >> b;
    b ^= c;
    b ^= 0b110;
    (b % 8) as u8
}

const fn execute_n_times<const N: usize>(initial_a: u64) -> [u8; N] {
    let mut a = initial_a;
    let mut res = [0; N];
    let mut idx = 0;
    while idx < 16 {
        res[idx] = execute_one_pass(a);
        a >>= 3;
        idx += 1;
    }
    res
}

fn part1(initial_state: State) -> String {
    let output = execute(initial_state, &PROGRAM);
    output
        .iter()
        .map(|&b| char::from_digit(b as u32, 10).unwrap())
        .intersperse(',')
        .collect::<String>()
}

fn part2() -> u64 {
    let mut candidates = VecDeque::<(u64, usize)>::new();
    for a in 0..0x3FF {
        if execute_one_pass(a) == PROGRAM[0] {
            candidates.push_back((a, 1));
        }
    }
    while let Some((candidate, idx)) = candidates.pop_front() {
        if idx == 16 {
            // now all of candidates should be at least 48 bit values of a
            break;
        }
        for head in 0..=7 {
            let next = head << (idx * 3 + 7) | candidate;
            let tail = next >> (idx * 3);
            if execute_one_pass(tail) == PROGRAM[idx] {
                candidates.push_back((next, idx + 1));
            }
        }
    }
    for &(candidate, idx) in &candidates {
        assert_eq!(16, idx);
        assert_eq!(PROGRAM, execute_n_times(candidate))
    }
    candidates.iter().map(|&(c, _)| c).min().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
}

fn execute(initial_state: State, program: &[u8]) -> Vec<u8> {
    let mut state = initial_state;
    let mut out = Vec::new();
    let as_combo = |operand: u8, state: State| match operand {
        ..=3 => operand as u64,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("invalid operand: {}", operand),
    };
    while state.ip < program.len() {
        let opcode = program[state.ip];
        let operand = program[state.ip + 1];
        let combo = as_combo(operand, state);
        match opcode {
            0 => {
                state.a >>= combo;
            }
            1 => {
                state.b ^= operand as u64;
            }
            2 => {
                state.b = combo & 0b111;
            }
            3 => {
                if state.a != 0 {
                    state.ip = operand as usize;
                    continue;
                }
            }
            4 => {
                state.b ^= state.c;
            }
            5 => {
                out.push((combo % 8) as u8);
            }
            6 => {
                state.b = state.a >> combo;
            }
            7 => {
                state.c = state.a >> combo;
            }
            _ => {
                panic!("Illegal opcode: {}", opcode);
            }
        }
        state.ip += 2;
    }
    out
}

#[test]
fn example() {
    let initial_state = State {
        a: 729,
        b: 0,
        c: 0,
        ip: 0,
    };
    let program = vec![0, 1, 5, 4, 3, 0];
    assert_eq!(
        vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0],
        execute(initial_state, &program)
    )
}
