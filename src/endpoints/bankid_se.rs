use crate::client::{IdkollenError, PollOptions, WaitError};
use crate::models::{AgeVerificationRequest, AgeVerificationStatus};
use crate::models::{
    BankIdSeAuthRequest, BankIdSePhoneAuthRequest, BankIdSePhoneSignRequest, BankIdSeSignRequest,
    BankIdSeStatus, BankIdSeVerifyRequest, BankIdSeVerifyResponse,
};
use std::time::Instant;

#[cfg(feature = "async")]
use crate::client::IdkollenClient;

#[cfg(feature = "async")]
pub struct BankIdSeEndpoint<'a>(pub(crate) &'a IdkollenClient);

#[cfg(feature = "async")]
impl BankIdSeEndpoint<'_> {
    /// Start a BankID SE authentication session.
    pub async fn auth(&self, req: BankIdSeAuthRequest) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/auth", &req).await
    }

    /// Start a BankID SE phone authentication session.
    pub async fn phone_auth(
        &self,
        req: BankIdSePhoneAuthRequest,
    ) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/phone/auth", &req).await
    }

    /// Start a BankID SE signing session.
    pub async fn sign(&self, req: BankIdSeSignRequest) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/sign", &req).await
    }

    /// Start a BankID SE phone signing session.
    pub async fn phone_sign(
        &self,
        req: BankIdSePhoneSignRequest,
    ) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/phone/sign", &req).await
    }

    /// Verify a scanned BankID SE QR code.
    pub async fn verify(
        &self,
        req: BankIdSeVerifyRequest,
    ) -> Result<BankIdSeVerifyResponse, IdkollenError> {
        self.0.post("/v3/bankid-se/verify", &req).await
    }

    /// Start a BankID SE age verification session.
    pub async fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/age-verification", &req).await
    }

    /// Poll the current status of a BankID SE authentication session.
    pub async fn auth_status(&self, id: &str) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.get(&format!("/v3/bankid-se/auth/{}", id)).await
    }

    /// Poll the current status of a BankID SE signing session.
    pub async fn sign_status(&self, id: &str) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.get(&format!("/v3/bankid-se/sign/{}", id)).await
    }

    /// Poll the current status of a BankID SE age verification session.
    pub async fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0
            .get(&format!("/v3/bankid-se/age-verification/{}", id))
            .await
    }

    /// Cancel a BankID SE authentication session.
    pub async fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/bankid-se/auth/{}", id)).await
    }

    /// Cancel a BankID SE signing session.
    pub async fn cancel_sign(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/bankid-se/sign/{}", id)).await
    }

    /// Cancel a BankID SE age verification session.
    pub async fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0
            .delete(&format!("/v3/bankid-se/age-verification/{}", id))
            .await
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub async fn wait_for_auth(
        &self,
        id: &str,
        opts: PollOptions,
    ) -> Result<BankIdSeStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id).await?;

            match status {
                BankIdSeStatus::Pending(_) => {
                    if Instant::now() >= deadline {
                        return Err(WaitError::Timeout);
                    }

                    tokio::time::sleep(opts.interval).await;
                },
                terminal => return Ok(terminal),
            }
        }
    }

    /// Poll until the signing session reaches a terminal state or the timeout elapses.
    pub async fn wait_for_sign(
        &self,
        id: &str,
        opts: PollOptions,
    ) -> Result<BankIdSeStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.sign_status(id).await?;

            match status {
                BankIdSeStatus::Pending(_) => {
                    if Instant::now() >= deadline {
                        return Err(WaitError::Timeout);
                    }

                    tokio::time::sleep(opts.interval).await;
                },
                terminal => return Ok(terminal),
            }
        }
    }

    /// Poll until the age verification session reaches a terminal state or the timeout elapses.
    pub async fn wait_for_age_verification(
        &self,
        id: &str,
        opts: PollOptions,
    ) -> Result<AgeVerificationStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.age_verification_status(id).await?;

            match status {
                AgeVerificationStatus::Pending(_) => {
                    if Instant::now() >= deadline {
                        return Err(WaitError::Timeout);
                    }

                    tokio::time::sleep(opts.interval).await;
                },
                terminal => return Ok(terminal),
            }
        }
    }
}

#[cfg(feature = "blocking")]
use crate::client::IdkollenBlockingClient;

#[cfg(feature = "blocking")]
pub struct BankIdSeBlockingEndpoint<'a>(pub(crate) &'a IdkollenBlockingClient);

#[cfg(feature = "blocking")]
impl BankIdSeBlockingEndpoint<'_> {
    /// Start a BankID SE authentication session.
    pub fn auth(&self, req: BankIdSeAuthRequest) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/auth", &req)
    }

    /// Start a BankID SE phone authentication session.
    pub fn phone_auth(
        &self,
        req: BankIdSePhoneAuthRequest,
    ) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/phone/auth", &req)
    }

    /// Start a BankID SE signing session.
    pub fn sign(&self, req: BankIdSeSignRequest) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/sign", &req)
    }

    /// Start a BankID SE phone signing session.
    pub fn phone_sign(
        &self,
        req: BankIdSePhoneSignRequest,
    ) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/phone/sign", &req)
    }

    /// Verify a scanned BankID SE QR code.
    pub fn verify(
        &self,
        req: BankIdSeVerifyRequest,
    ) -> Result<BankIdSeVerifyResponse, IdkollenError> {
        self.0.post("/v3/bankid-se/verify", &req)
    }

    /// Start a BankID SE age verification session.
    pub fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/bankid-se/age-verification", &req)
    }

    /// Poll the current status of a BankID SE authentication session.
    pub fn auth_status(&self, id: &str) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.get(&format!("/v3/bankid-se/auth/{}", id))
    }

    /// Poll the current status of a BankID SE signing session.
    pub fn sign_status(&self, id: &str) -> Result<BankIdSeStatus, IdkollenError> {
        self.0.get(&format!("/v3/bankid-se/sign/{}", id))
    }

    /// Poll the current status of a BankID SE age verification session.
    pub fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0
            .get(&format!("/v3/bankid-se/age-verification/{}", id))
    }

    /// Cancel a BankID SE authentication session.
    pub fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/bankid-se/auth/{}", id))
    }

    /// Cancel a BankID SE signing session.
    pub fn cancel_sign(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/bankid-se/sign/{}", id))
    }

    /// Cancel a BankID SE age verification session.
    pub fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0
            .delete(&format!("/v3/bankid-se/age-verification/{}", id))
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub fn wait_for_auth(&self, id: &str, opts: PollOptions) -> Result<BankIdSeStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id)?;

            match status {
                BankIdSeStatus::Pending(_) => {
                    if Instant::now() >= deadline {
                        return Err(WaitError::Timeout);
                    }

                    std::thread::sleep(opts.interval);
                },
                terminal => return Ok(terminal),
            }
        }
    }

    /// Poll until the signing session reaches a terminal state or the timeout elapses.
    pub fn wait_for_sign(&self, id: &str, opts: PollOptions) -> Result<BankIdSeStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.sign_status(id)?;

            match status {
                BankIdSeStatus::Pending(_) => {
                    if Instant::now() >= deadline {
                        return Err(WaitError::Timeout);
                    }

                    std::thread::sleep(opts.interval);
                },
                terminal => return Ok(terminal),
            }
        }
    }

    /// Poll until the age verification session reaches a terminal state or the timeout elapses.
    pub fn wait_for_age_verification(
        &self,
        id: &str,
        opts: PollOptions,
    ) -> Result<AgeVerificationStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.age_verification_status(id)?;

            match status {
                AgeVerificationStatus::Pending(_) => {
                    if Instant::now() >= deadline {
                        return Err(WaitError::Timeout);
                    }

                    std::thread::sleep(opts.interval);
                },
                terminal => return Ok(terminal),
            }
        }
    }
}
