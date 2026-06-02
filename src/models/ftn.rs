use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::ApiErrorCode;
use super::email::Email;
use super::ssn::Hetu;
use super::url::Url;

/// Request body for starting a Finnish Trust Network (FTN) authentication session.
#[must_use]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FtnAuthRequest {
    /// URL to redirect the user to after completing the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Url>,
    /// Request the user's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_phone: Option<bool>,
    /// Request the user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_email: Option<bool>,
    /// Request the user's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl FtnAuthRequest {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn redirect_url(mut self, url: Url) -> Self {
        self.redirect_url = Some(url);
        self
    }

    #[inline]
    pub fn request_phone(mut self, v: bool) -> Self {
        self.request_phone = Some(v);
        self
    }

    #[inline]
    pub fn request_email(mut self, v: bool) -> Self {
        self.request_email = Some(v);
        self
    }

    #[inline]
    pub fn request_address(mut self, v: bool) -> Self {
        self.request_address = Some(v);
        self
    }

    #[inline]
    pub fn ref_id(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }
}

/// FTN session status.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum FtnStatus {
    #[serde(rename = "PENDING")]
    Pending(FtnPending),
    #[serde(rename = "COMPLETED")]
    Completed(FtnCompleted),
    #[serde(rename = "FAILED")]
    Failed(FtnFailed),
}

/// Returned while the user has not yet acted.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FtnPending {
    pub id: String,
    pub ref_id: Option<String>,
    /// Redirect URL for the FTN provider login page.
    pub url: String,
}

/// Returned when the FTN session has completed successfully.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FtnCompleted {
    pub id: String,
    pub ref_id: Option<String>,
    /// Finnish personal identity code (henkilötunnus). May be absent depending on the provider.
    pub ssn: Option<Hetu>,
    pub name: String,
    pub given_name: String,
    pub surname: String,
    pub phone: Option<String>,
    pub email: Option<Email>,
    pub address: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub pid: Option<String>,
    pub bank_id: Option<String>,
}

/// Returned when the FTN session has failed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FtnFailed {
    pub id: String,
    pub ref_id: Option<String>,
    pub error: ApiErrorCode,
}
