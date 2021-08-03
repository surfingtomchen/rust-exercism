use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut hm: HashSet<u32> = HashSet::new();
    for f in factors {
        if *f == 0 { continue; }

        let mut i = 1;
        while i * f < limit {
            hm.insert(i * f);
            i += 1;
        }
    }
    hm.iter().sum()
}
