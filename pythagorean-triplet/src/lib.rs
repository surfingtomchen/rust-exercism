use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum / 3)
        .flat_map(|a| {
            ((a + 1)..(sum - a) / 2 + 1).filter_map(move |b| {
                if (a * a + b * b) == (sum - a - b) * (sum - a - b) {
                    Some([a, b, sum - a - b])
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<[u32; 3]>>()
}
