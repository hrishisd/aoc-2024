pub const fn count_lines(s: &[u8]) -> usize {
    if s.is_empty() {
        return 0;
    }
    let mut count = 1;
    let mut i = 0;

    while i < s.len() {
        if s[i] == b'\n' {
            count += 1;
        }
        i += 1;
    }

    // If the string ends with a newline, don't count an extra line
    if !s.is_empty() && s[s.len() - 1] == b'\n' {
        count -= 1;
    }

    count
}

#[test]
fn test_count_lines() {
    assert_eq!(0, count_lines(b""));
    assert_eq!(1, count_lines(b"asdfkljasdlkfjaslkjf"));
    assert_eq!(2, count_lines(b"line1\nline2"));
    assert_eq!(2, count_lines(b"line1\nline2\n"));
}
