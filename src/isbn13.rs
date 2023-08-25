use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum InvalidIsbn {
    TooLong,
    TooShort,
    FailedChecksum,
    InvalidCharacter,
}

impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len().cmp(&17) {
            Ordering::Less => Err(InvalidIsbn::TooShort),
            Ordering::Greater => Err(InvalidIsbn::TooLong),
            Ordering::Equal => {
                let digits = parse_digits(s).map_err(|_| InvalidIsbn::InvalidCharacter)?;

                let check = calculate_check_digit(&digits);

                if check == digits[12] {
                    Ok(Self {
                        raw: s.to_string(),
                        digits,
                    })
                } else {
                    Err(InvalidIsbn::FailedChecksum)
                }
            }
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let weighted_sum: u32 = digits
        .iter()
        .take(12)
        .zip([1, 3].repeat(6))
        .map(|(v, w)| *v as u32 * w)
        .sum();

    let calc = (10 - weighted_sum % 10) as u8;

    if calc == 10 { 0 } else { calc }
}

fn parse_digits(value: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut digits = Vec::with_capacity(13);
    for v in value.chars() {
        if v != '-' {
            digits.push(v.to_string().parse()?);
        }
    }

    Ok(digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_short() {
        assert_eq!(Err(InvalidIsbn::TooShort), Isbn::from_str("9783161484100"));
    }

    #[test]
    fn test_too_long() {
        assert_eq!(Err(InvalidIsbn::TooLong), Isbn::from_str("978-3-16-148410-00"));
    }

    #[test]
    fn test_invalid_character() {
        assert_eq!(Err(InvalidIsbn::InvalidCharacter), Isbn::from_str("978-3-16-14A10-00"));
    }

    #[test]
    fn test_invalid_check() {
        assert_eq!(Err(InvalidIsbn::FailedChecksum), Isbn::from_str("978-0-306-40615-1"));
    }

    #[test]
    fn test_parse() {
        let _ = Isbn::from_str("978-0-306-40615-0");
    }

    #[test]
    fn can_correctly_calculate_check_digits() {
        let cases = [
            ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
            ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
        ];

        for (case, check) in cases.iter() {
            let actual = calculate_check_digit(case);
            println!("{:?} -> {}?  {}", &case, check, actual);
            assert_eq!(calculate_check_digit(case), *check)
        }
    }

    #[test]
    fn rust_in_action() {
        let _: Isbn = "978-3-16-148410-0".parse().unwrap();
    }
}
