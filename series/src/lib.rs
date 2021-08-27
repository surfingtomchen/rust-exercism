pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .map(|s| String::from_utf8(s.to_vec()).unwrap())
        .collect()
}