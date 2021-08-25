use std::fmt::Display;

pub struct Luhn {
    str: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.str
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

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: Display,
{
    fn from(some: T) -> Self {
        Self {
            str: some.to_string(),
        }
    }
}
