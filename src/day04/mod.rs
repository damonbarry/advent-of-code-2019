#[derive(Debug)]
pub struct Password {
    value: String
}

impl Password {
    pub fn new(_value: &str) -> Result<Self, Error> {
        Err(Error { kind: ErrorKind::NotSixDigits })
    }
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    NotSixDigits,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_must_be_six_digits() {
        assert_eq!(Error { kind: ErrorKind::NotSixDigits }, Password::new("12345").unwrap_err());
        assert_eq!(Error { kind: ErrorKind::NotSixDigits }, Password::new("1234567").unwrap_err());
    }
}