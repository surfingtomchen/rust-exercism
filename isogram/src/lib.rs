pub fn check(candidate: &str) -> bool {
    let mut chars_exists = 0u64;
    candidate
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .all(|x| {
            let value = if x.is_ascii_uppercase() {
                x as u8 - 'A' as u8
            } else {
                x as u8 - 'a' as u8
            };
            let b = (chars_exists & 0x01 << value) > 0;
            chars_exists |= 0x01 << value;
            !b
        })
}
