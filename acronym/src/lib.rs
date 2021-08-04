pub fn abbreviate(phrase: &str) -> String {
    phrase.split(&[' ', '-'][..]).map(|str| str.trim_matches(|c: char| !c.is_ascii_alphabetic())).flat_map(|s| {
        s.chars().map(|c| c.to_ascii_uppercase()).take(1).chain(
            s.chars().skip_while(|c| c.is_ascii_uppercase()).filter(|c| c.is_ascii_uppercase()))
    }).collect()
}
