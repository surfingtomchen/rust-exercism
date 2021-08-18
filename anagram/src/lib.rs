use std::collections::HashSet;

pub fn hash(word: &str) -> (u64, u64) {
    let primes: [u64; 53] = [
        1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
        89, 87, 101, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
        73, 79, 83, 89, 87, 101,
    ];
    word.chars().fold((0, 0), |result, ch| {
        let u = match ch {
            'a'..='z' => ch as u8 + 1 - b'a',
            'A'..='Z' => ch as u8 + 1 - b'A',
            'Α'..='Ω' => (ch as u32 + 1 - 0x0391) as u8 + 26,
            'α'..='ω' => (ch as u32 + 1 - 0x03B1) as u8 + 26,
            _ => 0,
        };
        (
            result.0 | (0x01 << u),
            result.1 + primes[u as usize] + if u == 0 { ch as u64 } else { 0 },
        )
    })
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_hash = hash(word);
    possible_anagrams
        .iter()
        .filter(|s| s.to_lowercase() != word.to_lowercase() && s.len() == word.len())
        .filter_map(|s| (word_hash == hash(s)).then(|| *s))
        .collect()
}
