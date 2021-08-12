/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .fold(0, |s, ch| {
            s | 0x01 << (ch.to_ascii_lowercase() as u8 - b'a')
        })
        == 0x03FFFFFF
}
