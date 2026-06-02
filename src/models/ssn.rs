use fmt::Display;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use thiserror::Error;

// ── Swedish personnummer (Pno) ─────────────────────────────────────────────

/// A validated Swedish personal identity number (personnummer).
///
/// Accepts 10-digit (`YYMMDDXXXX`), 10-digit with separator (`YYMMDD-XXXX`,
/// `YYMMDD+XXXX`), 12-digit (`YYYYMMDDXXXX`), or 12-digit with separator
/// (`YYYYMMDD-XXXX`). Strips separators and validates the Luhn check digit.
/// Stored without separators; serializes/deserializes as a plain JSON string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pno(String);

#[derive(Debug, Error)]
#[error("invalid personnummer: {0}")]
pub struct PnoError(String);

impl Pno {
    pub fn parse(s: &str) -> Result<Self, PnoError> {
        let cleaned = s
            .chars()
            .filter(|&c| c != '-' && c != '+')
            .collect::<String>();

        if !cleaned.chars().all(|c| c.is_ascii_digit()) {
            return Err(PnoError("contains non-digit characters".to_owned()));
        }

        let ten = match cleaned.len() {
            10 => cleaned.as_str().to_owned(),
            12 => cleaned[2..].to_owned(),
            n => {
                return Err(PnoError(format!(
                    "expected 10 or 12 digits after stripping separators, got {n}"
                )));
            },
        };

        if !luhn10(&ten) {
            return Err(PnoError("Luhn check failed".to_owned()));
        }

        Ok(Self(cleaned))
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Pno> for String {
    #[inline]
    fn from(p: Pno) -> String {
        p.0
    }
}

impl AsRef<str> for Pno {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Pno {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Pno {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Pno {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;

        Pno::parse(&s).map_err(serde::de::Error::custom)
    }
}

// ── Norwegian fødselsnummer / D-number (Nnin) ─────────────────────────────

/// A validated Norwegian national identity number (fødselsnummer or D-number).
///
/// Accepts 11-digit strings, optionally with a dash at position 6. Validates
/// both modulo-11 control digits. Stored as 11 digits without separator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nnin(String);

#[derive(Debug, Error)]
#[error("invalid fødselsnummer: {0}")]
pub struct NninError(String);

impl Nnin {
    pub fn parse(s: &str) -> Result<Self, NninError> {
        let cleaned = s.chars().filter(|&c| c != '-').collect::<String>();

        if cleaned.len() != 11 {
            return Err(NninError(format!(
                "expected 11 digits, got {}",
                cleaned.len()
            )));
        }

        if !cleaned.chars().all(|c| c.is_ascii_digit()) {
            return Err(NninError("contains non-digit characters".to_owned()));
        }

        if !nnin_valid(&cleaned) {
            return Err(NninError("modulo-11 check failed".to_owned()));
        }

        Ok(Self(cleaned))
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Nnin> for String {
    #[inline]
    fn from(n: Nnin) -> String {
        n.0
    }
}

impl AsRef<str> for Nnin {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Nnin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Nnin {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Nnin {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;

        Nnin::parse(&s).map_err(serde::de::Error::custom)
    }
}

// ── Danish CPR number (Cpr) ───────────────────────────────────────────────

/// A validated Danish personal identification number (CPR-nummer).
///
/// Accepts 10-digit strings or the `DDMMYY-XXXX` form with a dash at position 6.
/// Validates that the date portion (DDMMYY) represents a plausible calendar date.
/// Stored as 10 digits without separator.
///
/// Note: the historical modulo-11 check is not applied because it was abolished
/// for persons born on or after 1 January 2007.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cpr(String);

#[derive(Debug, Error)]
#[error("invalid CPR number: {0}")]
pub struct CprError(String);

impl Cpr {
    pub fn parse(s: &str) -> Result<Self, CprError> {
        let cleaned = s.chars().filter(|&c| c != '-').collect::<String>();

        if cleaned.len() != 10 {
            return Err(CprError(format!(
                "expected 10 digits, got {}",
                cleaned.len()
            )));
        }

        if !cleaned.chars().all(|c| c.is_ascii_digit()) {
            return Err(CprError("contains non-digit characters".to_owned()));
        }

        let day: u32 = cleaned[..2].parse().unwrap();
        let month: u32 = cleaned[2..4].parse().unwrap();

        // Allow day 1-31 and day 61-91 for corrected CPRs; month 1-12.
        if !(1..=31).contains(&(day % 60)) || !(1..=12).contains(&month) {
            return Err(CprError(
                "date portion is not a valid calendar date".to_owned(),
            ));
        }

        Ok(Self(cleaned))
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Cpr> for String {
    #[inline]
    fn from(c: Cpr) -> String {
        c.0
    }
}

impl AsRef<str> for Cpr {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Cpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Cpr {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Cpr {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;

        Cpr::parse(&s).map_err(serde::de::Error::custom)
    }
}

// ── Finnish personal identity code (Hetu) ────────────────────────────────

/// A validated Finnish personal identity code (henkilötunnus, HETU).
///
/// Format: `DDMMYYcXXXK` where `c` is the century marker (`-` = 1900s,
/// `+` = 1800s, `A` = 2000s), `XXX` is an individual number, and `K` is
/// a check character from the alphabet `0123456789ABCDEFHJKLMNPRSTUVWXY`.
/// Stored in the original 11-character form.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hetu(String);

#[derive(Debug, Error)]
#[error("invalid henkilötunnus: {0}")]
pub struct HetuError(String);

const HETU_ALPHABET: &[u8] = b"0123456789ABCDEFHJKLMNPRSTUVWXY";

impl Hetu {
    pub fn parse(s: &str) -> Result<Self, HetuError> {
        let upper = s.to_ascii_uppercase();
        let b = upper.as_bytes();

        if b.len() != 11 {
            return Err(HetuError(format!(
                "expected 11 characters, got {}",
                b.len()
            )));
        }

        let sep = b[6] as char;

        if sep != '-' && sep != '+' && sep != 'A' {
            return Err(HetuError(format!(
                "invalid century marker '{sep}'; expected '-', '+', or 'A'"
            )));
        }

        let date_part = &upper[..6];
        let ind_part = &upper[7..10];

        if !date_part.chars().all(|c| c.is_ascii_digit()) {
            return Err(HetuError(
                "date portion contains non-digit characters".to_owned(),
            ));
        }

        if !ind_part.chars().all(|c| c.is_ascii_digit()) {
            return Err(HetuError(
                "individual number contains non-digit characters".to_owned(),
            ));
        }

        let n: u64 = format!("{date_part}{ind_part}").parse().unwrap();
        let expected = HETU_ALPHABET[(n % 31) as usize] as char;

        if b[10] as char != expected {
            return Err(HetuError(format!(
                "check character mismatch: got '{}', expected '{expected}'",
                b[10] as char
            )));
        }

        Ok(Self(upper))
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Hetu> for String {
    #[inline]
    fn from(h: Hetu) -> String {
        h.0
    }
}

impl AsRef<str> for Hetu {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Hetu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Hetu {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Hetu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;

        Hetu::parse(&s).map_err(serde::de::Error::custom)
    }
}

// ── Shared helpers ────────────────────────────────────────────────────────

/// Standard Luhn algorithm over a 10-digit ASCII string (weights 2,1 from left).
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

/// Norwegian fødselsnummer modulo-11 double-check-digit validation.
fn nnin_valid(s: &str) -> bool {
    let d = s
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let s1: u32 = [3u32, 7, 6, 1, 8, 9, 4, 5, 2, 1]
        .iter()
        .zip(d.iter())
        .map(|(w, v)| w * v)
        .sum();

    if !s1.is_multiple_of(11) {
        return false;
    }

    let s2: u32 = [5u32, 4, 3, 2, 7, 6, 5, 4, 3, 2, 1]
        .iter()
        .zip(d.iter())
        .map(|(w, v)| w * v)
        .sum();

    s2.is_multiple_of(11)
}
