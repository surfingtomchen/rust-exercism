#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sub<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if !first_list.is_empty() && second_list.is_empty() {
        return false;
    }

    let first_item = first_list.iter().next();
    let mut index = 0;
    if second_list
        .iter()
        .skip_while(|item| {
            index += 1;
            Some(*item) != first_item
        })
        .zip(first_list.iter())
        .filter(|(a, b)| a == b)
        .count()
        == first_list.len()
    {
        true
    } else {
        is_sub(first_list, &second_list[index..])
    }
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let is_sub_a_b = is_sub(first_list, second_list);
    let is_sub_b_a = is_sub(second_list, first_list);
    match (is_sub_a_b, is_sub_b_a) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
