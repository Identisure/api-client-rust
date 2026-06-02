use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::ApiErrorCode;
use super::email::Email;
use super::ssn::Cpr;
use super::url::Url;

/// Request body for starting a MitID authentication session.
#[must_use]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MitIdAuthRequest {
    /// URL to redirect the user to after completing the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Url>,
    /// Text shown to the user during authentication. Must not contain `%` or `<` (max 130 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_text: Option<String>,
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

impl MitIdAuthRequest {
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
    pub fn reference_text(mut self, v: impl Into<String>) -> Self {
        self.reference_text = Some(v.into());
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

/// Request body for starting a MitID backchannel authentication session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MitIdBackchannelAuthRequest {
    /// Danish CPR number.
    pub ssn: Cpr,
    /// Message displayed in the MitID app to bind the session.
    pub binding_message: String,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl MitIdBackchannelAuthRequest {
    #[inline]
    pub fn new(ssn: Cpr, binding_message: impl Into<String>) -> Self {
        Self {
            ssn,
            binding_message: binding_message.into(),
            callback_url: None,
            ref_id: None,
        }
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

/// Request body for starting a MitID signing session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MitIdSignRequest {
    /// Text to sign, displayed in MitID (max 600 chars).
    pub text: String,
    /// URL to redirect the user to after completing the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Url>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl MitIdSignRequest {
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            redirect_url: None,
            ref_id: None,
        }
    }

    #[inline]
    pub fn redirect_url(mut self, url: Url) -> Self {
        self.redirect_url = Some(url);
        self
    }

    #[inline]
    pub fn ref_id(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }
}

/// MitID session status.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum MitIdStatus {
    #[serde(rename = "PENDING")]
    Pending(MitIdPending),
    #[serde(rename = "COMPLETED")]
    Completed(MitIdCompleted),
    #[serde(rename = "FAILED")]
    Failed(MitIdFailed),
}

/// Returned while the user has not yet acted.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MitIdPending {
    pub id: String,
    pub ref_id: Option<String>,
    pub url: Option<String>,
    /// Present in the backchannel flow — display this to the user.
    pub binding_message: Option<String>,
}

/// Returned when the MitID session has completed successfully.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MitIdCompleted {
    pub id: String,
    pub ref_id: Option<String>,
    /// Danish CPR number.
    pub ssn: Cpr,
    pub name: String,
    pub given_name: String,
    pub surname: String,
    pub phone: Option<String>,
    pub email: Option<Email>,
    pub address: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub pid: Option<String>,
    pub bank_id: Option<String>,
    /// Present when the session was a signing session.
    pub sign_result: Option<MitIdSignResult>,
}

/// Returned when the MitID session has failed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MitIdFailed {
    pub id: String,
    pub ref_id: Option<String>,
    pub error: ApiErrorCode,
}

/// Signing result returned in a completed MitID sign session.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MitIdSignResult {
    pub checksum: String,
}
