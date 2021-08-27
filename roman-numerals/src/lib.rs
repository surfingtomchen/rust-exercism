use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let base = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut strings: Vec<&str> = vec![];
        let mut v = num;
        while v > 0 {
            let mut i = 0;
            while v < base[i].0 {
                i += 1;
            }
            strings.push(base[i].1);
            v -= base[i].0;
        }
        Self(strings.join(""))
    }
}
