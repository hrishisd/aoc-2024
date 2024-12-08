pub const fn count_lines(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }
    let mut count = 1;
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'\n' {
            count += 1;
        }
        i += 1;
    }

    // If the string ends with a newline, don't count an extra line
    if !bytes.is_empty() && bytes[bytes.len() - 1] == b'\n' {
        count -= 1;
    }

    count
}

#[test]
fn test_count_lines() {
    assert_eq!(0, count_lines(""));
    assert_eq!(1, count_lines("asdfkljasdlkfjaslkjf"));
    assert_eq!(2, count_lines("line1\nline2"));
    assert_eq!(2, count_lines("line1\nline2\n"));
}
