use core::iter;
use std::cmp::max;

pub fn encode(source: &str) -> String {
    let result: (u32, char, Vec<String>) = (0, '1', vec![]);
    source
        .chars()
        .chain(iter::once('1'))
        .fold(result, |mut r, c| {
            if c == r.1 {
                r.0 += 1;
            } else {
                if r.0 > 0 {
                    if r.0 > 1 {
                        r.2.push(r.0.to_string());
                    }
                    r.2.push(r.1.to_string());
                }
                r.0 = 1;
                r.1 = c;
            }
            r
        })
        .2
        .join("")
}

pub fn decode(source: &str) -> String {
    let result: (u32, Vec<char>) = (0, vec![]);
    source
        .chars()
        .fold(result, |mut r, c| {
            if c.is_numeric() {
                r.0 = r.0 * 10 + c.to_digit(10).unwrap();
            } else {
                r.1.extend(vec![c; max(1, r.0 as usize)]);
                r.0 = 0;
            }
            r
        })
        .1
        .iter()
        .collect()
}
