use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::common::{ApiErrorCode, CallInitiator};
use super::ip_address::IpAddress;
use super::org_number::OrgNumber;
use super::ssn::Pno;
use super::url::Url;

/// Request body for starting a BankID SE authentication session.
#[must_use]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSeAuthRequest {
    /// Swedish personal identification number. Restricts the session to this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<Pno>,
    /// End-user IP address (or the closest proxy address).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<IpAddress>,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Force PIN entry even when biometrics are enabled.
    pub pin_required: bool,
    /// Text describing the purpose of the identification, shown to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    /// Swedish organisation number — enables company signatory check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_number: Option<OrgNumber>,
    /// Fetch the user's registered address on completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl BankIdSeAuthRequest {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn ssn(mut self, ssn: Pno) -> Self {
        self.ssn = Some(ssn);
        self
    }

    #[inline]
    pub fn ip_address(mut self, ip: IpAddress) -> Self {
        self.ip_address = Some(ip);
        self
    }

    #[inline]
    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    #[inline]
    pub fn pin_required(mut self, required: bool) -> Self {
        self.pin_required = required;
        self
    }

    #[inline]
    pub fn intent(mut self, intent: impl Into<String>) -> Self {
        self.intent = Some(intent.into());
        self
    }

    #[inline]
    pub fn org_number(mut self, org_number: OrgNumber) -> Self {
        self.org_number = Some(org_number);
        self
    }

    #[inline]
    pub fn request_address(mut self, request: bool) -> Self {
        self.request_address = Some(request);
        self
    }

    #[inline]
    pub fn ref_id(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }
}

/// Request body for starting a BankID SE signing session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSeSignRequest {
    /// Visible text the user must approve in BankID (max 50 000 chars).
    pub text: String,
    /// Restrict the signing session to this Swedish personal number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<Pno>,
    /// End-user IP address (or the closest proxy address).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<IpAddress>,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Force PIN entry even when biometrics are enabled.
    pub pin_required: bool,
    /// Hash digest of an associated file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// Swedish organisation number — enables company signatory check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_number: Option<OrgNumber>,
    /// Fetch the user's registered address on completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl BankIdSeSignRequest {
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ssn: None,
            ip_address: None,
            callback_url: None,
            pin_required: false,
            digest: None,
            org_number: None,
            request_address: None,
            ref_id: None,
        }
    }

    #[inline]
    pub fn ssn(mut self, ssn: Pno) -> Self {
        self.ssn = Some(ssn);
        self
    }

    #[inline]
    pub fn ip_address(mut self, ip: IpAddress) -> Self {
        self.ip_address = Some(ip);
        self
    }

    #[inline]
    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    #[inline]
    pub fn pin_required(mut self, required: bool) -> Self {
        self.pin_required = required;
        self
    }

    #[inline]
    pub fn digest(mut self, digest: impl Into<String>) -> Self {
        self.digest = Some(digest.into());
        self
    }

    #[inline]
    pub fn org_number(mut self, org_number: OrgNumber) -> Self {
        self.org_number = Some(org_number);
        self
    }

    #[inline]
    pub fn request_address(mut self, request: bool) -> Self {
        self.request_address = Some(request);
        self
    }

    #[inline]
    pub fn ref_id(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }
}

/// Request body for starting a BankID SE phone authentication session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSePhoneAuthRequest {
    /// Swedish personal identification number of the user to authenticate.
    pub ssn: Pno,
    /// Whether the user or the RP initiated the phone call.
    pub call_initiator: CallInitiator,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Force PIN entry even when biometrics are enabled.
    pub pin_required: bool,
    /// Text describing the purpose of the identification, shown to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    /// Swedish organisation number — enables company signatory check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_number: Option<OrgNumber>,
    /// Fetch the user's registered address on completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl BankIdSePhoneAuthRequest {
    #[inline]
    pub fn new(ssn: Pno, call_initiator: CallInitiator) -> Self {
        Self {
            ssn,
            call_initiator,
            callback_url: None,
            pin_required: false,
            intent: None,
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
    pub fn pin_required(mut self, required: bool) -> Self {
        self.pin_required = required;
        self
    }

    #[inline]
    pub fn intent(mut self, intent: impl Into<String>) -> Self {
        self.intent = Some(intent.into());
        self
    }

    #[inline]
    pub fn org_number(mut self, org_number: OrgNumber) -> Self {
        self.org_number = Some(org_number);
        self
    }

    #[inline]
    pub fn request_address(mut self, request: bool) -> Self {
        self.request_address = Some(request);
        self
    }

    #[inline]
    pub fn ref_id(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }
}

/// Request body for starting a BankID SE phone signing session.
#[must_use]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSePhoneSignRequest {
    /// Swedish personal identification number of the user to sign.
    pub ssn: Pno,
    /// Whether the user or the RP initiated the phone call.
    pub call_initiator: CallInitiator,
    /// Visible text the user must approve in BankID (max 50 000 chars).
    pub text: String,
    /// URL to receive the result callback on success or failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Url>,
    /// Force PIN entry even when biometrics are enabled.
    pub pin_required: bool,
    /// Hash digest of an associated file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// Swedish organisation number — enables company signatory check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_number: Option<OrgNumber>,
    /// Fetch the user's registered address on completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_address: Option<bool>,
    /// Reference ID returned verbatim in the result and callback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

impl BankIdSePhoneSignRequest {
    pub fn new(ssn: Pno, call_initiator: CallInitiator, text: impl Into<String>) -> Self {
        Self {
            ssn,
            call_initiator,
            text: text.into(),
            callback_url: None,
            pin_required: false,
            digest: None,
            org_number: None,
            request_address: None,
            ref_id: None,
        }
    }

    pub fn callback_url(mut self, url: Url) -> Self {
        self.callback_url = Some(url);
        self
    }

    pub fn pin_required(mut self, required: bool) -> Self {
        self.pin_required = required;
        self
    }

    pub fn digest(mut self, digest: impl Into<String>) -> Self {
        self.digest = Some(digest.into());
        self
    }

    pub fn org_number(mut self, org_number: OrgNumber) -> Self {
        self.org_number = Some(org_number);
        self
    }

    pub fn request_address(mut self, request: bool) -> Self {
        self.request_address = Some(request);
        self
    }

    pub fn ref_id(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }
}

/// Request body for verifying a scanned BankID SE QR code.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSeVerifyRequest {
    /// Complete content of the scanned BankID QR code.
    pub qr_code: String,
}

impl BankIdSeVerifyRequest {
    pub fn new(qr_code: impl Into<String>) -> Self {
        Self {
            qr_code: qr_code.into(),
        }
    }
}

/// BankID SE session status.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum BankIdSeStatus {
    #[serde(rename = "PENDING")]
    Pending(BankIdSePending),
    #[serde(rename = "COMPLETED")]
    Completed(BankIdSeCompleted),
    #[serde(rename = "FAILED")]
    Failed(BankIdSeFailed),
}

/// Returned while the user has not yet acted in the BankID app.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSePending {
    /// BankID order reference / session ID.
    pub id: String,
    /// Reference ID returned verbatim in the result and callback.
    pub ref_id: Option<String>,
    /// Token for launching the BankID app directly (autostart URL). Absent in phone-auth flows.
    pub auto_start_token: Option<String>,
    /// Static token used to seed the animated QR code. Absent in phone-auth flows.
    pub qr_start_token: Option<String>,
    /// Secret used together with `qr_start_token` to generate QR frames. Absent in phone-auth flows.
    pub qr_start_secret: Option<String>,
    /// BankID hint code describing the current waiting state.
    pub hint_code: Option<String>,
}

/// Returned when the BankID session has completed successfully.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSeCompleted {
    /// BankID order reference / session ID.
    pub id: String,
    /// Reference ID returned verbatim in the result and callback.
    pub ref_id: Option<String>,
    /// Swedish personal identification number (personnummer).
    pub ssn: Pno,
    pub name: String,
    pub given_name: String,
    pub surname: String,
    /// Date the BankID certificate became valid (YYYY-MM-DD).
    pub cert_start_date: Option<NaiveDate>,
    /// Present only when `request_address` was `true`.
    pub address: Option<String>,
    /// Company signatory result text. Present only when `org_number` was provided.
    pub company_signatory_text: Option<String>,
}

/// Returned when the BankID session has failed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSeFailed {
    /// BankID order reference / session ID.
    pub id: String,
    /// Reference ID returned verbatim in the result and callback.
    pub ref_id: Option<String>,
    pub error: ApiErrorCode,
}

/// Response from the BankID SE QR code verification endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankIdSeVerifyResponse {
    /// Swedish personal identification number.
    pub ssn: Pno,
    pub name: String,
    pub given_name: String,
    pub surname: String,
    pub age: Option<u8>,
    /// Date the QR code was verified (YYYY-MM-DD, UTC).
    pub verified_at: Option<NaiveDate>,
}
