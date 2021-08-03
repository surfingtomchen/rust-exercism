pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes: Vec<u32> = vec![];
    let mut p_index = 0;
    let mut number = 3;

    while primes.len() < n as usize {
        if p_index >= primes.len() || primes[p_index] * primes[p_index] > number {
            primes.push(number);
            number += 2;
            p_index = 0;
        } else if number % primes[p_index] == 0 {
            number += 2;
            p_index = 0;
        } else {
            p_index += 1;
        }
    }

    primes[primes.len() - 1]
}
