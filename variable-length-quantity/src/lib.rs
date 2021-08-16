#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

fn value_to_bytes(value: u32) -> Vec<u8> {
    let mut v = value;
    let mut result: Vec<u8> = vec![];
    result.push((v & 0x7F) as u8);
    while v >> 7 > 0 {
        v = v >> 7;
        result.push((v & 0x7F | 0x80) as u8);
    }
    result.reverse();
    result
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| value_to_bytes(*v)).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let (v, _, end) = bytes.iter().fold(Ok((vec![], 0u64, false)), |v, u| {
        v.and_then(|(mut v, mut sum, _)| {
            let mut end = false;
            sum = (sum << 7) + (u & 0x7f) as u64;
            if sum > u32::MAX as u64 {
                return Err(Error::Overflow);
            }
            if u & 0x80 == 0 {
                v.push(sum as u32);
                sum = 0;
                end = true;
            }
            Ok((v, sum, end))
        })
    })?;
    if !end {
        Err(Error::IncompleteNumber)
    } else {
        Ok(v)
    }
}
