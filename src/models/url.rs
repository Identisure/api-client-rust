use fmt::Display;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use thiserror::Error;

/// A validated URL.
///
/// Constructed via [`Url::parse`]; serializes/deserializes as a plain JSON string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Url(String);

#[derive(Debug, Error)]
#[error("invalid URL: {0}")]
pub struct UrlError(#[from] url::ParseError);

impl Url {
    pub fn parse(s: &str) -> Result<Self, UrlError> {
        url::Url::parse(s)?;

        Ok(Self(s.to_owned()))
    }

    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Url> for String {
    #[inline]
    fn from(u: Url) -> String {
        u.0
    }
}

impl AsRef<str> for Url {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Url {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Url::parse(&s).map_err(serde::de::Error::custom)
    }
}
