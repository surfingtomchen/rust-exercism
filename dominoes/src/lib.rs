use std::iter;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut input_set = input
        .into_iter()
        .cloned()
        .zip(iter::repeat(false))
        .collect::<Vec<((u8, u8), bool)>>();
    seek_solution(&mut input_set, None, None)
}

pub fn seek_solution(
    input: &mut [((u8, u8), bool)],
    start: Option<u8>,
    end: Option<u8>,
) -> Option<Vec<(u8, u8)>> {
    let iter = input.to_vec().clone();
    let mut taken_count = 0;
    for (i, ((a, b), taken)) in iter.iter().enumerate() {
        if *taken {
            taken_count += 1;
            continue;
        }
        input[i].1 = true;

        match (start, *a, *b) {
            (None, _, _) => {
                if let Some(mut result) = seek_solution(input, Some(*b), Some(*a)) {
                    result.insert(0, (*a, *b));
                    return Some(result);
                }
            }
            (Some(s), a, b) => {
                if a == s {
                    if let Some(mut result) = seek_solution(input, Some(b), end) {
                        result.insert(0, (a, b));
                        return Some(result);
                    }
                }

                if a != b && b == s {
                    if let Some(mut result) = seek_solution(input, Some(a), end) {
                        result.insert(0, (b, a));
                        return Some(result);
                    }
                }
            }
        }

        input[i].1 = false;
    }

    if taken_count == input.len() && start == end {
        Some(vec![])
    } else {
        None
    }
}
