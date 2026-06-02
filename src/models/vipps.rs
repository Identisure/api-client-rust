use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::ApiErrorCode;
use super::email::Email;
use super::ssn::Nnin;
use super::url::Url;

/// Request body for starting a Vipps MobilePay authentication session.
#[must_use]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VippsAuthRequest {
    /// URL to redirect the user to after completing the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Url>,
    /// Request the user's Norwegian personal number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_ssn: Option<bool>,
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
    /// (BETA) Deep-link URI to return the user to your app after authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_callback_uri: Option<Url>,
}

impl VippsAuthRequest {
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
    pub fn request_ssn(mut self, v: bool) -> Self {
        self.request_ssn = Some(v);
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

    #[inline]
    pub fn app_callback_uri(mut self, uri: Url) -> Self {
        self.app_callback_uri = Some(uri);
        self
    }
}

/// Request body for starting a Vipps MobilePay backchannel authentication session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VippsBackchannelAuthRequest {
    /// Phone number of the user to authenticate.
    pub phone: String,
    /// Request the user's Norwegian personal number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_ssn: Option<bool>,
    /// Request the user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_email: Option<bool>,
    /// Request the user's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl VippsBackchannelAuthRequest {
    #[inline]
    pub fn new(phone: impl Into<String>) -> Self {
        Self {
            phone: phone.into(),
            request_ssn: None,
            request_email: None,
            request_address: None,
            callback_url: None,
            ref_id: None,
        }
    }

    #[inline]
    pub fn request_ssn(mut self, v: bool) -> Self {
        self.request_ssn = Some(v);
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
    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    #[inline]
    pub fn ref_id(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }
}

/// Vipps MobilePay session status.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum VippsStatus {
    #[serde(rename = "PENDING")]
    Pending(VippsPending),
    #[serde(rename = "COMPLETED")]
    Completed(VippsCompleted),
    #[serde(rename = "FAILED")]
    Failed(VippsFailed),
}

/// Returned while the user has not yet acted.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VippsPending {
    pub id: String,
    pub ref_id: Option<String>,
    /// Redirect URL for the Vipps login page.
    pub url: Option<String>,
}

/// Returned when the Vipps session has completed successfully.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VippsCompleted {
    pub id: String,
    pub ref_id: Option<String>,
    /// Present when `request_ssn` was `true`.
    pub ssn: Option<Nnin>,
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

/// Returned when the Vipps session has failed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VippsFailed {
    pub id: String,
    pub ref_id: Option<String>,
    pub error: ApiErrorCode,
}
