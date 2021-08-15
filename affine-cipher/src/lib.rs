use std::cmp::{max, min};

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const MOD: i32 = 26;

pub fn find_mmi(a: i32) -> Result<i32, AffineCipherError> {
    let mut m = max(a, MOD);
    let mut n = min(a, MOD);
    let mut q;
    let mut r;
    let (mut t1, mut t2) = (0, 1);
    while n > 0 {
        q = m / n;
        r = m % n;
        m = n;
        n = r;
        let temp = t2;
        t2 = t1 - q * t2;
        t1 = temp;
    }
    if m > 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(if t1 > 0 { t1 } else { MOD + t1 })
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    find_mmi(a)?;
    let chunks = plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_numeric())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let x = c as u8 - b'a';
                (((a * x as i32 + b) % MOD) as u8 + b'a') as char
            } else {
                c
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|s| s.iter().collect())
        .collect::<Vec<String>>();
    Ok(chunks.join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mmi = find_mmi(a)?;
    Ok(ciphertext
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_numeric())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let y = (c as u8 - b'a') as i32;
                let cc = ((y - b) * mmi % MOD) as i32;
                if cc >= 0 {
                    (cc as u8 + b'a') as char
                } else {
                    (cc + MOD + b'a' as i32) as u8 as char
                }
            } else {
                c
            }
        })
        .collect())
}
