use rand::random;

pub fn encode(key: &str, source: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    source
        .chars()
        .into_iter()
        .zip(key.chars().cycle())
        .map(|(s, k)| match (k, s) {
            ('a'..='z', 'a'..='z') => {
                Some(((k as u8 - b'a' + (s as u8 - b'a')) % 26 + b'a') as char)
            }
            _ => None,
        })
        .collect()
}

pub fn decode(key: &str, source: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    source
        .chars()
        .into_iter()
        .zip(key.chars().into_iter().cycle())
        .map(|(s, k)| match (k, s) {
            ('a'..='z', 'a'..='z') => {
                Some((((s as u8 - b'a') + 26 - (k as u8 - b'a')) % 26 + b'a') as char)
            }
            _ => None,
        })
        .collect()
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..100)
        .into_iter()
        .map(|_| (random::<u8>() % 26 + b'a') as char)
        .collect::<String>();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
