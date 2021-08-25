/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut double = false;
    if let Some(result) = code
        .chars()
        .rev()
        .map(|c| {
            if c.is_numeric() || c == ' ' {
                Some(c)
            } else {
                None
            }
        })
        .fold(Some((0, 0)), |n, c| {
            n.and_then(|n| {
                c.and_then(|c| {
                    if c.is_numeric() {
                        let mut value = (c as u8 - b'0') as u32;

                        if double && value != 9 {
                            value = value * 2 % 9;
                        }
                        double = !double;
                        Some((n.0 + value, n.1 + 1))
                    } else {
                        Some(n)
                    }
                })
            })
        })
    {
        result.1 > 1 && result.0 % 10 == 0
    } else {
        false
    }
}
