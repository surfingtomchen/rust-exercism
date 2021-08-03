pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut v: Vec<u64> = vec![];
    let mut p = 2;
    loop {
        if n == 1 { break; }

        if n % p == 0 {
            v.push(p);
            n = n / p;
        } else if p * p > n {
            v.push(n);
            break;
        } else {
            p += if p == 2 { 1 } else { 2 }
        }
    }

    v
}