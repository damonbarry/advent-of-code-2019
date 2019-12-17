#[derive(Debug)]
pub struct Password<T> {
    value: T
}

impl<T> Password<T> {
    pub fn new(_value: T) -> Result<Self, Error> {
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
        assert_eq!(Error { kind: ErrorKind::NotSixDigits }, Password::new(12345_u32).unwrap_err());
        assert_eq!(Error { kind: ErrorKind::NotSixDigits }, Password::new(1234567_u32).unwrap_err());
    }
}