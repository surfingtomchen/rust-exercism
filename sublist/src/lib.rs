use crate::Comparison::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Less => {
            if contains(second_list, first_list) {
                Sublist
            } else {
                Unequal
            }
        }
        Ordering::Equal => {
            if first_list == second_list {
                Equal
            } else {
                Unequal
            }
        }
        Ordering::Greater => {
            if contains(first_list, second_list) {
                Superlist
            } else {
                Unequal
            }
        }
    }
}

/// if a contains b
pub fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|list| list == b)
}
