#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    let mut actual_value = number
        .iter()
        .rev()
        .fold(Ok(0), |result, n| {
            result.and_then(|v| {
                (*n < from_base)
                    .then(|| v * from_base + n)
                    .ok_or(Error::InvalidDigit(*n))
            })
        })?;

    let mut to_vec: Vec<u32> = vec![];
    while actual_value != 0 {
        to_vec.push(actual_value % to_base);
        actual_value = actual_value / to_base;
    }
    to_vec.reverse();
    if to_vec.is_empty() {
        to_vec.push(0);
    }

    Ok(to_vec)
}
