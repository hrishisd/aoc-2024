use std::{collections::HashSet, fmt::Debug, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    L,
    R,
    U,
    D,
    A,
}

fn main() {
    // let inputs: [(u64, [&str; 1]); 5] = [
    //     // 029A
    //     (29, ["<A^A^^>AvvvA"]),
    //     // 980A
    //     (980, ["^^^A<AvvvA>A"]),
    //     // 179A
    //     (179, ["^<<A^^A>>AvvvA"]),
    //     // 456A
    //     (456, ["^^<<A>A>AvvA"]),
    //     // 379A
    //     (379, ["^A<<^^A>>AvvvA"]),
    // ];

    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+

    let inputs: [(u64, &[&str]); 5] = [
        (169, &["^<<A^>>A^AvvvA", "^<<A>>^A^AvvvA"]),
        (279, &[
            "<^A<^^A>>AvvvA",
            "^<A<^^A>>AvvvA",
            "<^A^^<A>>AvvvA",
            "^<A^^<A>>AvvvA",
        ]),
        (540, &["^^<A<A>vvA>A", "<^^A<A>vvA>A"]),
        (869, &[
            "<^^^A>vA^AvvvA",
            "<^^^Av>A^AvvvA",
            "^^^<A>vA^AvvvA",
            "^^^<Av>A^AvvvA",
        ]),
        (789, &["^^^<<A>A>AvvvA"]),
    ];

    let mut res = 0;
    for (num, strs) in inputs {
        // println!("Solving for {num}");
        let min_len = strs.iter().map(|s| solve(s) as u64).min().unwrap();
        res += min_len * num;
    }
    println!("part 1: {res}");
}

/// find min length of final sequence that produces input sequence
fn solve(sequence: &str) -> usize {
    // println!("sequence: {sequence:?}");
    let sequence = parse(sequence.as_bytes());
    let mut frontier = vec![sequence];
    for _ in 0..2 {
        // println!("{i}: {}", frontier.len());
        let mut next = vec![];
        for input in frontier {
            for sequence in produce(&input) {
                next.push(sequence);
            }
        }
        frontier = next;
    }
    // println!("produced {} sequences", frontier.len());
    frontier.iter().map(Vec::len).min().unwrap()
}

/// returns all min length sequences of key presses which produce the desired output sequence
fn produce(output: &[Key]) -> HashSet<Vec<Key>> {
    let mut robot_state = Key::A;
    let mut accs = HashSet::from([vec![]]);
    for &key in output {
        let mut next_accs: HashSet<Vec<Key>> = HashSet::new();
        for acc in accs {
            for &presses in robot_state.shortest_paths_to(key) {
                let mut clone = acc.clone();
                clone.extend(presses);
                clone.push(Key::A);
                next_accs.insert(clone);
            }
        }
        accs = next_accs;
        robot_state = key;
    }
    let min_len = accs.iter().map(Vec::len).min().unwrap();
    accs.retain(|seq| seq.len() == min_len);
    accs
}

impl Key {
    /// Find the set of shortest paths from the current key to the target key.
    ///
    /// Prefer repetition, e.g. >>^ is preferable to >^>
    const fn shortest_paths_to(self, target: Self) -> &'static [&'static [Self]] {
        use Key::{A, D, L, R, U};
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        match (self, target) {
            (L, L) | (R, R) | (A, A) | (U, U) | (D, D) => &[&[]],
            (L, R) => &[&[R, R]],
            (L, U) => &[&[R, U]],
            (L, D) => &[&[R]],
            // (L, A) => &[&[R, R, U], &[R, U, R]],
            (L, A) => &[&[R, R, U]],
            (R, L) => &[&[L, L]],
            (R, U) => &[&[U, L], &[L, U]],
            (R, D) => &[&[L]],
            (R, A) => &[&[U]],
            (U, L) => &[&[D, L]],
            (U, R) => &[&[R, D], &[D, R]],
            (U, D) => &[&[D]],
            (U, A) => &[&[R]],
            (D, L) => &[&[L]],
            (D, R) => &[&[R]],
            (D, U) => &[&[U]],
            (D, A) => &[&[U, R], &[R, U]],
            // (A, L) => &[&[D, L, L], &[L, D, L]],
            (A, L) => &[&[D, L, L]],
            (A, R) => &[&[D]],
            (A, U) => &[&[L]],
            (A, D) => &[&[D, L], &[L, D]],
        }
    }

    const fn as_char(self) -> char {
        use Key::{A, D, L, R, U};
        match self {
            L => '<',
            R => '>',
            U => '^',
            D => 'v',
            A => 'A',
        }
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.as_char();
        write!(f, "{c}")
    }
}

fn parse(s: &[u8]) -> Vec<Key> {
    use Key::{A, D, L, R, U};
    s.iter()
        .map(|b| match b {
            b'^' => U,
            b'v' => D,
            b'<' => L,
            b'>' => R,
            b'A' => A,
            _ => panic!("Illegal key: {b}"),
        })
        .collect()
}
