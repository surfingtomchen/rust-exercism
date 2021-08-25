pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
            .rev()
            .filter(|c| *c != ' ')
            .try_fold((0, 0), |(value, count), ch| {
                ch.to_digit(10)
                    .map(|x| if count % 2 == 0 { x } else { x * 2 })
                    .map(|x| if x > 9 { x - 9 } else { x })
                    .map(|x| (value + x, count + 1))
            })
            .map_or(false, |(value, count)| value % 10 == 0 && count > 1)
    }
}
