/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    decode(plain)
        .chars()
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|x| x.is_ascii_alphabetic() || x.is_numeric())
        .map(|x| {
            if x.is_ascii_alphabetic() {
                (2 * b'a' + 25 - x.to_ascii_lowercase() as u8) as char
            } else {
                x
            }
        })
        .collect::<String>()
}
