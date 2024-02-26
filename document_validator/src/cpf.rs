use std::{iter::repeat, sync::Arc};

use crate::Document;

const CPF_LENGTH: usize = 11;

/// Represents the brazilian document CPF, which uniquely identifies a brazilian
/// citizen.
///
/// # Thread safety
/// The string data of the document is stored in an `Arc<str>`, so this struct
/// implements `Send` and `Sync`.
#[derive(Clone, Debug)]
pub struct Cpf(Arc<str>);

impl Document for Cpf {
    fn new(document: &str) -> Option<Self>
    where
        Self: Sized,
    {
        Self::validate(document).then(|| Self(format_cpf(document).into()))
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
            CPF_LENGTH => digits,
            x if x < CPF_LENGTH => repeat(0)
                .take(CPF_LENGTH - x)
                .chain(digits.iter().copied())
                .collect(),
            _ => return false,
        };

        if digits.iter().all(|x| *x == digits[0]) {
            return false;
        }

        let first_digit = next(&digits[..CPF_LENGTH - 2], CPF_LENGTH - 2);

        if first_digit != digits[CPF_LENGTH - 2] {
            return false;
        }

        let second_digit = next(&digits[..CPF_LENGTH - 1], CPF_LENGTH - 1);
        second_digit == digits[CPF_LENGTH - 1]
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

fn next(digits: &[u8], len: usize) -> u8 {
    let weighted_sum: usize = digits
        .iter()
        .enumerate()
        .map(|(i, v)| (len - i + 1) * usize::from(*v))
        .sum();

    match 11 - (weighted_sum % 11) {
        // There is no truncation, since is between 0 and 9 (inclusive)
        #[allow(clippy::cast_possible_truncation)]
        x @ 0..=9 => x as u8,
        _ => 0,
    }
}

fn format_cpf(document: &str) -> String {
    let digits = document
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();

    format!("{digits:0>11}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cpf_constructor() {
        let Some(cpf) = Cpf::new("434") else {
            panic!("Faild to construct cpf")
        };

        assert_eq!(cpf.as_str(), "00000000434");
    }

    #[test]
    fn formatted_cpf() {
        assert_eq!(
            format_cpf("4-34"),
            "00000000434",
            "Failed to remove mask and left pad"
        );
    }

    #[test]
    fn valid_cpf() {
        assert!(
            Cpf::validate("00000000434"),
            "Failed to validate a valid cpf without mask"
        );
        assert!(
            Cpf::validate("000.000.004-34"),
            "Failed to validate a valid cpf with mask"
        );

        assert!(
            Cpf::validate("434"),
            "Failed to validate a cpf without leading zeroes"
        );
    }

    #[test]
    fn invalid_cpf() {
        assert!(!Cpf::validate("00000000433"), "This is not a valid cpf");
        assert!(!Cpf::validate("00000001434"), "This is not a valid cpf");

        assert!(
            !Cpf::validate("00000000000"),
            "Though mathematically valid, a cpf cannot be 11 equal digits"
        );
    }
}
