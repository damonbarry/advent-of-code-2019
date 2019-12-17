#[derive(Debug)]
pub struct Password {
    value: String
}

impl Password {
    pub fn new(value: &str) -> Result<Self, Error> {
        if !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(Error::new(ErrorKind::NotANumber));
        }

        if value.len() != 6 {
            return Err(Error::new(ErrorKind::NotSixDigits));
        }

        Ok(Password { value: value.to_owned() })
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_characters_must_be_numeric() {
        assert_eq!(Error::new(ErrorKind::NotANumber), Password::new("12345z").unwrap_err());
        assert_eq!(Error::new(ErrorKind::NotANumber), Password::new("abcdef").unwrap_err());
        assert_eq!(Error::new(ErrorKind::NotANumber), Password::new("123-45").unwrap_err());
        assert_eq!(Error::new(ErrorKind::NotANumber), Password::new("12.345").unwrap_err());
        assert_eq!(Error::new(ErrorKind::NotANumber), Password::new("-12345").unwrap_err());
    }

    #[test]
    fn password_must_be_six_digits() {
        assert_eq!(Error::new(ErrorKind::NotSixDigits), Password::new("12345").unwrap_err());
        assert_eq!(Error::new(ErrorKind::NotSixDigits), Password::new("1234567").unwrap_err());
        assert_eq!(Error::new(ErrorKind::NotSixDigits), Password::new("0123456").unwrap_err());
    }
}