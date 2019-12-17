use std::ops::RangeBounds;

const LOWER_BOUND: u32 = 307_237;
const UPPER_BOUND: u32 = 769_058;

#[derive(Debug, PartialEq)]
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

        if !value.chars().zip(value.chars().skip(1)).any(|z| z.0 == z.1) {
            return Err(Error::new(ErrorKind::NoDoubles));
        }

        let mut cur: u32 = 0;
        for ch in value.chars() {
            let next = ch.to_digit(10).unwrap();
            if next < cur {
                return Err(Error::new(ErrorKind::DigitsDecrease));
            }
            cur = next;
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
    DigitsDecrease,
    NoDoubles,
    NotANumber,
    NotSixDigits,
    OutOfRange,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_if_any_char_is_not_a_number() {
        assert_eq!(
            Err(Error::new(ErrorKind::NotANumber)),
            Password::new("12345z")
        );

        assert_eq!(
            Err(Error::new(ErrorKind::NotANumber)),
            Password::new("abcdef")
        );

        assert_eq!(
            Err(Error::new(ErrorKind::NotANumber)),
            Password::new("123-45")
        );

        assert_eq!(
            Err(Error::new(ErrorKind::NotANumber)),
            Password::new("12.345")
        );

        assert_eq!(
            Err(Error::new(ErrorKind::NotANumber)),
            Password::new("-12345")
        );
    }

    #[test]
    fn fails_if_password_is_not_six_digits() {
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
    fn fails_if_password_is_not_in_the_given_range() {
        assert_eq!(
            Error::new(ErrorKind::OutOfRange),
            Password::new_with_range("000004", 0..4).unwrap_err()
        );

        assert_eq!(
            Error::new(ErrorKind::OutOfRange),
            Password::new_with_range("000005", 0..=4).unwrap_err()
        );
    }

    #[test]
    fn fails_if_no_two_adjacent_digits_are_the_same() {
        assert_eq!(
            Err(Error::new(ErrorKind::NoDoubles)),
            Password::new_with_range("123789", 100000..200000)
        );
    }

    #[test]
    fn fails_if_successive_digits_decrease() {
        assert_eq!(
            Err(Error::new(ErrorKind::DigitsDecrease)),
            Password::new_with_range("223450", 200000..300000)
        );
    }

    #[test]
    fn succeeds_if_password_meets_all_criteria() {
        assert_eq!(
            Ok(Password {
                value: "345677".to_owned()
            }),
            Password::new("345677")
        );

        assert_eq!(
            Ok(Password {
                value: "111111".to_owned()
            }),
            Password::new_with_range("111111", 100000..200000)
        );
    }
}
