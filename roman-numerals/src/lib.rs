use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.0.chars().rev().collect::<String>())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let base = [("I", "V"), ("X", "L"), ("C", "D"), ("M", "M")];
        let s = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .zip(0..)
            .map(|(c, index)| match c {
                1..=3 => base[index].0.repeat(c as usize),
                4 => base[index].1.to_owned() + base[index].0,
                5 => base[index].1.to_owned(),
                6..=8 => base[index].0.repeat(c as usize - 5) + base[index].1,
                9 => base[index + 1].0.to_owned() + base[index].0,
                _ => "".to_owned(),
            })
            .collect::<String>();
        Self(s)
    }
}
