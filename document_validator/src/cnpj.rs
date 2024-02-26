use std::{iter::repeat, sync::Arc};

use crate::Document;

const CNPJ_LENGTH: usize = 14;

/// Represents the brazilian document CNPJ, which uniquely identifies a brazilian
/// company.
///
/// # Thread safety
/// The string data of the document is stored in an `Arc<str>`, so this struct
/// implements `Send` and `Sync`.
#[derive(Clone, Debug)]
pub struct Cnpj(Arc<str>);

impl Document for Cnpj {
    fn new(document: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Self::validate(document).then(|| Self(format_cnpj(document).into()))
    }

    fn validate(document: &str) -> bool
    where
        Self: Sized,
    {
        let digits: Box<[_]> = document
            .bytes()
            .filter(u8::is_ascii_digit)
            .map(|x| x - b'0')
            .collect();

        let digits = match digits.len() {
            CNPJ_LENGTH => digits,
            x if x < CNPJ_LENGTH => repeat(0)
                .take(CNPJ_LENGTH - x)
                .chain(digits.iter().copied())
                .collect(),
            _ => return false,
        };

        if digits.iter().all(|x| *x == digits[0]) {
            return false;
        }

        let first_digit = next(&digits[..CNPJ_LENGTH - 2], CNPJ_LENGTH - 2);

        if first_digit != digits[CNPJ_LENGTH - 2] {
            return false;
        }

        let second_digit = next(&digits[..CNPJ_LENGTH - 1], CNPJ_LENGTH - 1);
        second_digit == digits[CNPJ_LENGTH - 1]
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

fn next(digits: &[u8], len: usize) -> u8 {
    let weighted_sum: usize = digits
        .iter()
        .enumerate()
        .map(|(i, v)| ((len - i - 1) % 8 + 2) * usize::from(*v))
        .sum();

    match 11 - (weighted_sum % 11) {
        // There is no truncation, since is between 0 and 9 (inclusive)
        #[allow(clippy::cast_possible_truncation)]
        x @ 0..=9 => x as u8,
        _ => 0,
    }
}

fn format_cnpj(document: &str) -> String {
    let digits = document
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();

    format!("{digits:0>14}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cnpj_constructor() {
        let Some(cnpj) = Cnpj::new("08.254.798/0001-00") else {
            panic!("Faild to construct cnpj")
        };

        assert_eq!(cnpj.as_str(), "08254798000100");
    }

    #[test]
    fn formatted_cnpj() {
        assert_eq!(
            format_cnpj("8.254.798/0001-00"),
            "08254798000100",
            "Failed to remove mask and left pad"
        );
    }

    #[test]
    fn valid_cnpj() {
        assert!(
            Cnpj::validate("08254798000100"),
            "Failed to validate a valid cnpj without mask"
        );
        assert!(
            Cnpj::validate("08.254.798/0001-00"),
            "Failed to validate a valid cnpj with mask"
        );

        assert!(
            Cnpj::validate("08254798000100"),
            "Failed to validate a cnpj without leading zeroes"
        );
    }

    #[test]
    fn invalid_cnpj() {
        assert!(!Cnpj::validate("08254798000101"), "This is not a valid cnpj");
        assert!(!Cnpj::validate("08254788000100"), "This is not a valid cnpj");

        assert!(
            !Cnpj::validate("00000000000000"),
            "Though mathematically valid, a cnpj cannot be 11 equal digits"
        );
    }
}

