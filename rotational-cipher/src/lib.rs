pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as i8;
            if c.is_ascii_alphabetic() {
                ((c as i8 - base + key + 26) % 26 + base) as u8 as char
            } else {
                c
            }
        })
        .collect()
}
