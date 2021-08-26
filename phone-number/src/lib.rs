pub fn number(user_number: &str) -> Option<String> {
    user_number
        .chars()
        .filter(|ch| ch.is_numeric())
        .skip_while(|ch| *ch == '1')
        .zip(0..)
        .map(|(digit, index)| match (digit, index) {
            ('0'..='1', 0) | ('0'..='1', 3) => None,
            _ => Some(digit),
        })
        .collect::<Option<String>>()
        .map_or(None, |str| (str.len() == 10).then(|| str))
}