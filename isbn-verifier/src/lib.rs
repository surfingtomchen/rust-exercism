/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|x| *x != '-')
        .fold((10i32, Some(0i32)), |mut sum, c| {
            sum.1 = sum.1.and_then(|s| match sum.0 {
                2..=10 => c
                    .is_numeric()
                    .then(|| (s + sum.0 * c.to_digit(10).unwrap() as i32) % 11),
                1 => (c.is_numeric() || c == 'x' || c == 'X')
                    .then(|| (s + c.to_digit(10).unwrap_or(10) as i32) % 11),
                _ => None,
            });
            sum.0 -= 1;
            sum
        })
        == (0, Some(0))
}
