use crate::Error::{InvalidDigit, SpanTooLong};

#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        _ => string_digits
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map_or(Err(InvalidDigit(c)), |v| Ok(v as u64))
            })
            .collect::<Result<Vec<_>, Error>>()?
            .windows(span)
            .map(|x| x.iter().product())
            .max()
            .ok_or(SpanTooLong),
    }
}
