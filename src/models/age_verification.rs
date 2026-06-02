use serde::{Deserialize, Serialize};

use super::common::ApiErrorCode;
use super::url::Url;

/// Request body for starting an age verification session.
///
/// At least one of `min_age` or `max_age` must be provided.
/// If both are given, `max_age` must be >= `min_age`.
#[must_use]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeVerificationRequest {
    /// Minimum age (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_age: Option<u32>,
    /// Maximum age (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u32>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// URL to redirect the user to after completing age verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Url>,
}

impl AgeVerificationRequest {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn min_age(mut self, age: u32) -> Self {
        self.min_age = Some(age);
        self
    }

    #[inline]
    pub fn max_age(mut self, age: u32) -> Self {
        self.max_age = Some(age);
        self
    }

    #[inline]
    pub fn ref_id(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }

    #[inline]
    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    #[inline]
    pub fn redirect_url(mut self, url: Url) -> Self {
        self.redirect_url = Some(url);
        self
    }
}

/// Age verification session status.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum AgeVerificationStatus {
    #[serde(rename = "PENDING")]
    Pending(AgeVerificationPending),
    #[serde(rename = "COMPLETED")]
    Completed(AgeVerificationCompleted),
    #[serde(rename = "FAILED")]
    Failed(AgeVerificationFailed),
}

/// Returned while the user has not yet completed age verification.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeVerificationPending {
    pub id: String,
    pub url: Option<String>,
    pub min_age: Option<u32>,
    pub max_age: Option<u32>,
}

/// Returned when the age verification session has completed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeVerificationCompleted {
    pub id: String,
    /// `true` if the user's age is within the requested range.
    pub age_verified: bool,
}

/// Returned when the age verification session has failed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeVerificationFailed {
    pub id: String,
    pub error: ApiErrorCode,
}
