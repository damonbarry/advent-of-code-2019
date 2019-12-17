use std::ops::RangeBounds;

const LOWER_BOUND: u32 = 307_237;
const UPPER_BOUND: u32 = 769_058;

#[derive(Debug)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: &str) -> Result<Self, Error> {
        Password::new_with_range(value, LOWER_BOUND..=UPPER_BOUND)
    }

    pub fn new_with_range<T: RangeBounds<u32>>(value: &str, range: T) -> Result<Self, Error> {
        if !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(Error::new(ErrorKind::NotANumber));
        }

        if value.len() != 6 {
            return Err(Error::new(ErrorKind::NotSixDigits));
        }

        let number = value.parse::<u32>().unwrap();
        if !range.contains(&number) {
            return Err(Error::new(ErrorKind::OutOfRange));
        }

        Ok(Password {
            value: value.to_owned(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorKind {
    NotANumber,
    NotSixDigits,
    OutOfRange,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_characters_must_be_numeric() {
        assert_eq!(
            Error::new(ErrorKind::NotANumber),
            Password::new("12345z").unwrap_err()
        );
        assert_eq!(
            Error::new(ErrorKind::NotANumber),
            Password::new("abcdef").unwrap_err()
        );
        assert_eq!(
            Error::new(ErrorKind::NotANumber),
            Password::new("123-45").unwrap_err()
        );
        assert_eq!(
            Error::new(ErrorKind::NotANumber),
            Password::new("12.345").unwrap_err()
        );
        assert_eq!(
            Error::new(ErrorKind::NotANumber),
            Password::new("-12345").unwrap_err()
        );
    }

    #[test]
    fn password_must_be_six_digits() {
        assert_eq!(
            Error::new(ErrorKind::NotSixDigits),
            Password::new("12345").unwrap_err()
        );
        assert_eq!(
            Error::new(ErrorKind::NotSixDigits),
            Password::new("1234567").unwrap_err()
        );
        assert_eq!(
            Error::new(ErrorKind::NotSixDigits),
            Password::new("0123456").unwrap_err()
        );
    }

    #[test]
    fn password_must_be_in_the_given_range() {
        assert_eq!(
            Error::new(ErrorKind::OutOfRange),
            Password::new_with_range("000004", 0..4).unwrap_err()
        );
        assert_eq!(
            Error::new(ErrorKind::OutOfRange),
            Password::new_with_range("000005", 0..=4).unwrap_err()
        );
    }
}
