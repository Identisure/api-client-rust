use fmt::Display;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use thiserror::Error;

/// A validated Swedish organisation number (organisationsnummer).
///
/// Accepts `XXXXXX-XXXX` or `XXXXXXXXXX` (10 digits). Validates the Luhn check digit.
/// Stored in normalised `XXXXXX-XXXX` form; serializes/deserializes as a plain JSON string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrgNumber(String);

#[derive(Debug, Error)]
#[error("invalid organisation number: {0}")]
pub struct OrgNumberError(String);

impl OrgNumber {
    pub fn parse(s: &str) -> Result<Self, OrgNumberError> {
        let digits = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();

        if digits.len() != 10 {
            return Err(OrgNumberError(format!(
                "must contain exactly 10 digits, got {}",
                digits.len()
            )));
        }

        // Third digit must be ≥ 2 (distinguishes org numbers from personal numbers).
        if digits.as_bytes()[2] < b'2' {
            return Err(OrgNumberError(
                "third digit must be 2 or greater".to_owned(),
            ));
        }

        if !luhn10(&digits) {
            return Err(OrgNumberError("Luhn check failed".to_owned()));
        }

        Ok(Self(format!("{}-{}", &digits[..6], &digits[6..])))
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Standard Luhn algorithm over a 10-digit ASCII string.
fn luhn10(s: &str) -> bool {
    let sum: u32 = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let d = c.to_digit(10).unwrap();
            let v = if i % 2 == 0 { d * 2 } else { d };
            if v >= 10 { v - 9 } else { v }
        })
        .sum();

    sum.is_multiple_of(10)
}

impl From<OrgNumber> for String {
    #[inline]
    fn from(o: OrgNumber) -> String {
        o.0
    }
}

impl AsRef<str> for OrgNumber {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for OrgNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for OrgNumber {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for OrgNumber {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;

        OrgNumber::parse(&s).map_err(serde::de::Error::custom)
    }
}
