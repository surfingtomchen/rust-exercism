use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|ch: char| !ch.is_alphanumeric() && ch != '\'')
        .filter(|str| !str.is_empty())
        .map(|str| str.trim_matches('\''))
        .fold(HashMap::new(), |mut hm, str| {
            *hm.entry(str.to_lowercase()).or_insert(0) += 1;
            hm
        })
}
