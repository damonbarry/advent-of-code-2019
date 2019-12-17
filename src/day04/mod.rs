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

        let mut cch = 0_u32;
        let mut prev: char = 'x';
        for ch in value.chars() {
            if ch == prev {
                cch += 1;
            } else if cch == 2 {
                break;
            } else {
                cch = 1;
            }

            prev = ch;
        }

        if cch != 2 {
            return Err(Error::new(ErrorKind::NoDoubles));
        }

        let mut prev: u32 = 0;
        for ch in value.chars() {
            let cur = ch.to_digit(10).unwrap();
            if cur < prev {
                return Err(Error::new(ErrorKind::DigitsDecrease));
            }
            prev = cur;
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
            Password::new_with_range("123789", 100_000..200_000)
        );
    }

    #[test]
    fn fails_if_no_matching_adjacent_digits_are_pairs() {
        assert_eq!(
            Err(Error::new(ErrorKind::NoDoubles)),
            Password::new_with_range("123444", 100_000..200_000)
        );
    }

    #[test]
    fn fails_if_successive_digits_decrease() {
        assert_eq!(
            Err(Error::new(ErrorKind::DigitsDecrease)),
            Password::new_with_range("223450", 200_000..300_000)
        );
    }

    #[test]
    fn succeeds_if_password_meets_all_criteria() {
        assert_eq!(
            Ok(Password {
                value: "345567".to_owned()
            }),
            Password::new("345567")
        );

        assert_eq!(
            Ok(Password {
                value: "112233".to_owned()
            }),
            Password::new_with_range("112233", 100_000..200_000)
        );

        assert_eq!(
            Ok(Password {
                value: "111122".to_owned()
            }),
            Password::new_with_range("111122", 100_000..200_000)
        );
    }

    #[test]
    fn solve_day4_problem() {
        let mut candidates = Vec::<String>::new();

        for num in LOWER_BOUND..=UPPER_BOUND {
            let password = num.to_string();
            if Password::new(&password).is_ok() {
                candidates.push(password);
            }
        }

        assert_eq!(589, candidates.len());
    }
}
