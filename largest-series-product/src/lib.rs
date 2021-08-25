use crate::Error::InvalidDigit;

#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    if span == 0 {
        return Ok(1);
    }

    Ok(string_digits
        .as_bytes()
        .windows(span)
        .map(|slice| {
            slice.iter().map(|c| *c as char).try_fold(1u64, |total, c| {
                c.to_digit(10)
                    .map_or(Err(InvalidDigit(c)), |v| Ok(total * v as u64))
            })
        })
        .collect::<Result<Vec<u64>, Error>>()?
        .iter()
        .cloned()
        .max()
        .unwrap_or(1))
}
