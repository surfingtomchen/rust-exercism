use crate::Classification::{Abundant, Deficient, Perfect};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        1 => Some(Deficient),
        _ => match (2..=(num as f64).sqrt() as u64)
            .into_iter()
            .fold(1, |sum, n| {
                (num % n == 0)
                    .then(|| sum + n + if num / n == n { 0 } else { num / n })
                    .unwrap_or(sum)
            })
            .cmp(&num)
        {
            Ordering::Less => Some(Deficient),
            Ordering::Equal => Some(Perfect),
            Ordering::Greater => Some(Abundant),
        },
    }
}
