use fmt::Display;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use thiserror::Error;

/// A validated email address.
///
/// Constructed via [`Email::parse`]; serializes/deserializes as a plain JSON string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

#[derive(Debug, Error)]
#[error("invalid email address: {0}")]
pub struct EmailError(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, EmailError> {
        s.parse::<email_address::EmailAddress>()
            .map_err(|e| EmailError(e.to_string()))?;

        Ok(Self(s.to_owned()))
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    #[inline]
    fn from(e: Email) -> String {
        e.0
    }
}

impl AsRef<str> for Email {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Email {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;

        Email::parse(&s).map_err(serde::de::Error::custom)
    }
}
