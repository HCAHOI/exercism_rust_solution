#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }
    for c in string_digits.chars() {
        if !c.is_numeric() {
            return Err(Error::InvalidDigit(c));
        }
    }
    let digits = string_digits
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();

    let &res = digits
        .windows(span)
        .map(|w| w.into_iter().product())
        .collect::<Vec<u64>>()
        .iter()
        .max()
        .unwrap();

    Ok(res)
}
