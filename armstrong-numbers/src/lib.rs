pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 { return true; };

    let mut v: Vec<u32> = vec![];
    let mut n = num;
    while n != 0 {
        v.push(n % 10);
        n = n / 10;
    }

    let sum = v.iter().map(|x| x.pow(v.len() as u32)).sum::<u32>() as u64;
    sum == num as u64
}
