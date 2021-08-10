#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    // implement your palindrome type here
    pub value: u64,
    pub factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Self {
            value: a * b,
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }

    pub fn is_palindrome(n: &u64) -> bool {
        let mut x = *n;
        let mut reverse = 0;
        while x != 0 {
            reverse = reverse * 10 + x % 10;
            x = x / 10;
        }
        *n == reverse
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let result: Option<(Palindrome, Palindrome)> = None;
    (min..=max)
        .into_iter()
        .flat_map(|x| {
            (min..=max)
                .into_iter()
                .filter(|y| *y >= x)
                .map(|y| (x * y, x, y))
                .collect::<Vec<(u64, u64, u64)>>()
        })
        .filter(|(n, _, _)| Palindrome::is_palindrome(n))
        .fold(result, |result, (n, x, y)| {
            if let Some((mut min, mut max)) = result {
                if min.value > n {
                    min = Palindrome::new(x, y);
                } else if min.value == n {
                    min.insert(x, y);
                }

                if max.value < n {
                    max = Palindrome::new(x, y);
                } else if max.value == n {
                    max.insert(x, y);
                }

                Some((min, max))
            } else {
                Some((Palindrome::new(x, y), Palindrome::new(x, y)))
            }
        })
}
