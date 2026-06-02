use fmt::Display;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::net::IpAddr;
use thiserror::Error;

/// A validated IPv4 or IPv6 address.
///
/// Constructed via [`IpAddress::parse`]; serializes/deserializes as a plain JSON string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IpAddress(IpAddr);

#[derive(Debug, Error)]
#[error("invalid IP address: {0}")]
pub struct IpAddressError(#[from] std::net::AddrParseError);

impl IpAddress {
    #[inline]
    pub fn parse(s: &str) -> Result<Self, IpAddressError> {
        Ok(Self(s.parse::<IpAddr>()?))
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> IpAddr {
        self.0
    }
}

impl From<IpAddr> for IpAddress {
    #[inline]
    fn from(addr: IpAddr) -> Self {
        Self(addr)
    }
}

impl From<IpAddress> for IpAddr {
    #[inline]
    fn from(a: IpAddress) -> IpAddr {
        a.0
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Serialize for IpAddress {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        // IpAddr serializes as a string in JSON
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for IpAddress {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        IpAddr::deserialize(d).map(Self)
    }
}
