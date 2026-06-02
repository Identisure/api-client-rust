use serde::{Deserialize, Serialize};

use super::common::{ApiErrorCode, Country};
use super::org_number::OrgNumber;
use super::url::Url;

/// Minimum required Freja eID registration level.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrejaRegistrationLevel {
    /// Standard Freja eID registration (default).
    #[serde(rename = "EXTENDED")]
    Extended,
    /// Enhanced Freja eID+ registration.
    #[serde(rename = "PLUS")]
    Plus,
}

impl Default for FrejaRegistrationLevel {
    #[inline]
    fn default() -> Self {
        Self::Extended
    }
}

/// Request body for starting a Freja eID authentication session.
#[must_use]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrejaAuthRequest {
    /// Personal number of the user to authenticate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Minimum required Freja registration level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_registration_level: Option<FrejaRegistrationLevel>,
    /// Organisation number — enables company signatory check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_number: Option<OrgNumber>,
    /// Fetch the user's registered address on completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl FrejaAuthRequest {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn ssn(mut self, v: impl Into<String>) -> Self {
        self.ssn = Some(v.into());
        self
    }

    #[inline]
    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    #[inline]
    pub fn min_registration_level(mut self, v: FrejaRegistrationLevel) -> Self {
        self.min_registration_level = Some(v);
        self
    }

    #[inline]
    pub fn org_number(mut self, org_number: OrgNumber) -> Self {
        self.org_number = Some(org_number);
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

/// Request body for starting a Freja eID backchannel authentication session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrejaBackchannelAuthRequest {
    /// Personal number of the user to authenticate.
    pub ssn: String,
    /// Country of the user's Freja identity document.
    pub country: Country,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Minimum required Freja registration level.
    pub min_registration_level: FrejaRegistrationLevel,
    /// Organisation number — enables company signatory check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_number: Option<OrgNumber>,
    /// Fetch the user's registered address on completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl FrejaBackchannelAuthRequest {
    #[inline]
    pub fn new(
        ssn: impl Into<String>,
        country: Country,
        min_registration_level: FrejaRegistrationLevel,
    ) -> Self {
        Self {
            ssn: ssn.into(),
            country,
            min_registration_level,
            callback_url: None,
            org_number: None,
            request_address: None,
            ref_id: None,
        }
    }

    #[inline]
    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    #[inline]
    pub fn org_number(mut self, org_number: OrgNumber) -> Self {
        self.org_number = Some(org_number);
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

/// Request body for starting a Freja eID signing session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrejaSignRequest {
    /// Text to sign, displayed to the user in the Freja app.
    pub text: String,
    /// Personal number of the user to sign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Minimum required Freja registration level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_registration_level: Option<FrejaRegistrationLevel>,
    /// Organisation number — enables company signatory check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_number: Option<OrgNumber>,
    /// Fetch the user's registered address on completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl FrejaSignRequest {
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ssn: None,
            callback_url: None,
            min_registration_level: None,
            org_number: None,
            request_address: None,
            ref_id: None,
        }
    }

    #[inline]
    pub fn ssn(mut self, v: impl Into<String>) -> Self {
        self.ssn = Some(v.into());
        self
    }

    #[inline]
    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    #[inline]
    pub fn min_registration_level(mut self, v: FrejaRegistrationLevel) -> Self {
        self.min_registration_level = Some(v);
        self
    }

    #[inline]
    pub fn org_number(mut self, org_number: OrgNumber) -> Self {
        self.org_number = Some(org_number);
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

/// Request body for starting a Freja eID backchannel signing session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrejaBackchannelSignRequest {
    /// Personal number of the user to sign.
    pub ssn: String,
    /// Country of the user's Freja identity document.
    pub country: Country,
    /// Text to sign, displayed to the user in the Freja app.
    pub text: String,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Minimum required Freja registration level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_registration_level: Option<FrejaRegistrationLevel>,
    /// Organisation number — enables company signatory check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_number: Option<OrgNumber>,
    /// Fetch the user's registered address on completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl FrejaBackchannelSignRequest {
    #[inline]
    pub fn new(ssn: impl Into<String>, country: Country, text: impl Into<String>) -> Self {
        Self {
            ssn: ssn.into(),
            country,
            text: text.into(),
            callback_url: None,
            min_registration_level: None,
            org_number: None,
            request_address: None,
            ref_id: None,
        }
    }

    #[inline]
    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    #[inline]
    pub fn min_registration_level(mut self, v: FrejaRegistrationLevel) -> Self {
        self.min_registration_level = Some(v);
        self
    }

    #[inline]
    pub fn org_number(mut self, org_number: OrgNumber) -> Self {
        self.org_number = Some(org_number);
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

/// Freja eID session status.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum FrejaStatus {
    #[serde(rename = "PENDING")]
    Pending(FrejaPending),
    #[serde(rename = "COMPLETED")]
    Completed(FrejaCompleted),
    #[serde(rename = "FAILED")]
    Failed(FrejaFailed),
}

/// Returned while the user has not yet acted in the Freja app.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrejaPending {
    pub id: String,
    pub ref_id: Option<String>,
    /// Freja transaction reference — also used as the autostart token. Absent in backchannel flows.
    pub auto_start_token: Option<String>,
    /// Data string to render as the Freja QR code. Absent in backchannel flows.
    pub qr_data: Option<String>,
}

/// Returned when the Freja eID session has completed successfully.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrejaCompleted {
    pub id: String,
    pub ref_id: Option<String>,
    pub ssn: String,
    /// Country of the user's identity document (e.g. `SWEDEN`).
    pub country: Country,
    pub name: String,
    pub given_name: String,
    pub surname: String,
    /// Present only when `request_address` was `true`.
    pub address: Option<String>,
    /// Present only when `org_number` was provided.
    pub company_signatory_text: Option<String>,
}

/// Returned when the Freja eID session has failed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrejaFailed {
    pub id: String,
    pub ref_id: Option<String>,
    pub error: ApiErrorCode,
}
