use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::ApiErrorCode;
use super::email::Email;
use super::ssn::Nnin;
use super::url::Url;

/// Request body for starting a BankID NO authentication session.
#[must_use]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdNoAuthRequest {
    /// URL to redirect the user to after completing the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Url>,
    /// Request the user's Norwegian personal number (fødselsnummer).
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

impl BankIdNoAuthRequest {
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

/// Request body for starting a BankID NO backchannel authentication session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdNoBackchannelAuthRequest {
    /// Norwegian personal number (fødselsnummer).
    pub ssn: Nnin,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl BankIdNoBackchannelAuthRequest {
    #[inline]
    pub fn new(ssn: Nnin) -> Self {
        Self {
            ssn,
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

/// Request body for starting a BankID NO signing session.
#[must_use]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdNoSignRequest {
    /// URL to redirect the user to after completing the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Url>,
    /// Text to sign (max 118 chars). Mutually exclusive with `documents`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Document IDs to sign (from `/v3/document`). Mutually exclusive with `text`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_ssn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_phone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl BankIdNoSignRequest {
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
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.text = Some(v.into());
        self
    }

    #[inline]
    pub fn documents(mut self, v: Vec<String>) -> Self {
        self.documents = Some(v);
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
}

/// BankID NO session status.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
#[expect(clippy::large_enum_variant)]
pub enum BankIdNoStatus {
    #[serde(rename = "PENDING")]
    Pending(BankIdNoPending),
    #[serde(rename = "COMPLETED")]
    Completed(BankIdNoCompleted),
    #[serde(rename = "FAILED")]
    Failed(BankIdNoFailed),
}

/// Returned while the user has not yet acted.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdNoPending {
    pub id: String,
    pub ref_id: Option<String>,
    /// Redirect URL for the browser-flow login page.
    pub url: Option<String>,
    /// Present in the backchannel flow — display this to the user.
    pub binding_message: Option<String>,
}

/// Returned when the BankID NO session has completed successfully.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdNoCompleted {
    pub id: String,
    pub ref_id: Option<String>,
    /// Norwegian personal number. Present when `request_ssn` was `true`.
    pub ssn: Option<Nnin>,
    pub name: String,
    pub given_name: String,
    pub surname: String,
    /// Present when `request_phone` was `true`.
    pub phone: Option<String>,
    /// Present when `request_email` was `true`.
    pub email: Option<Email>,
    /// Present when `request_address` was `true`.
    pub address: Option<String>,
    pub birth_date: Option<NaiveDate>,
    /// BankID PID.
    pub pid: Option<String>,
    pub bank_id: Option<String>,
    /// Present when the session was a signing session.
    pub sign_result: Option<BankIdNoSignResult>,
    /// Present when documents were signed.
    pub signed_documents: Option<Vec<BankIdNoSignedDocument>>,
}

/// Returned when the BankID NO session has failed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdNoFailed {
    pub id: String,
    pub ref_id: Option<String>,
    pub error: ApiErrorCode,
}

/// Signing result returned in a completed BankID NO sign session.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdNoSignResult {
    pub end_user: String,
    pub merchant: String,
    pub hash: String,
}

/// A signed document reference returned in a completed BankID NO sign session.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdNoSignedDocument {
    /// Document UUID matching the uploaded document.
    pub id: String,
    /// SHA hash of the signed PDF.
    pub hash: String,
}
