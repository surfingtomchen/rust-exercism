pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut v: Vec<u64> = (0..=upper_bound).collect();
    for i in 2..=(upper_bound as f64).sqrt() as usize {
        if v[i] == 0 { continue; }
        for j in (i * 2..=upper_bound as usize).step_by(i) {
            v[j] = 0;
        }
    }

    v.iter().skip(2).filter(|x| **x != 0).map(|x| *x).collect()
}
