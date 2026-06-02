use crate::client::{IdkollenError, PollOptions, WaitError};
use crate::models::{AgeVerificationRequest, AgeVerificationStatus};
use crate::models::{
    BankIdNoAuthRequest, BankIdNoBackchannelAuthRequest, BankIdNoSignRequest, BankIdNoStatus,
};
use std::time::Instant;

#[cfg(feature = "async")]
use crate::client::IdkollenClient;

#[cfg(feature = "async")]
pub struct BankIdNoEndpoint<'a>(pub(crate) &'a IdkollenClient);

#[cfg(feature = "async")]
impl BankIdNoEndpoint<'_> {
    /// Start a BankID NO authentication session.
    pub async fn auth(&self, req: BankIdNoAuthRequest) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.post("/v3/bankid-no/auth", &req).await
    }

    /// Start a BankID NO backchannel authentication session.
    pub async fn backchannel_auth(
        &self,
        req: BankIdNoBackchannelAuthRequest,
    ) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.post("/v3/bankid-no/backchannel/auth", &req).await
    }

    /// Start a BankID NO signing session.
    pub async fn sign(&self, req: BankIdNoSignRequest) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.post("/v3/bankid-no/sign", &req).await
    }

    /// Start a BankID NO age verification session.
    pub async fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/bankid-no/age-verification", &req).await
    }

    /// Poll the current status of a BankID NO authentication session.
    pub async fn auth_status(&self, id: &str) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.get(&format!("/v3/bankid-no/auth/{}", id)).await
    }

    /// Poll the current status of a BankID NO signing session.
    pub async fn sign_status(&self, id: &str) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.get(&format!("/v3/bankid-no/sign/{}", id)).await
    }

    /// Poll the current status of a BankID NO age verification session.
    pub async fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0
            .get(&format!("/v3/bankid-no/age-verification/{}", id))
            .await
    }

    /// Cancel a BankID NO authentication session.
    pub async fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/bankid-no/auth/{}", id)).await
    }

    /// Cancel a BankID NO signing session.
    pub async fn cancel_sign(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/bankid-no/sign/{}", id)).await
    }

    /// Cancel a BankID NO age verification session.
    pub async fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0
            .delete(&format!("/v3/bankid-no/age-verification/{}", id))
            .await
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub async fn wait_for_auth(
        &self,
        id: &str,
        opts: PollOptions,
    ) -> Result<BankIdNoStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id).await?;

            match status {
                BankIdNoStatus::Pending(_) => {
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
    ) -> Result<BankIdNoStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.sign_status(id).await?;

            match status {
                BankIdNoStatus::Pending(_) => {
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
pub struct BankIdNoBlockingEndpoint<'a>(pub(crate) &'a IdkollenBlockingClient);

#[cfg(feature = "blocking")]
impl BankIdNoBlockingEndpoint<'_> {
    /// Start a BankID NO authentication session.
    pub fn auth(&self, req: BankIdNoAuthRequest) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.post("/v3/bankid-no/auth", &req)
    }

    /// Start a BankID NO backchannel authentication session.
    pub fn backchannel_auth(
        &self,
        req: BankIdNoBackchannelAuthRequest,
    ) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.post("/v3/bankid-no/backchannel/auth", &req)
    }

    /// Start a BankID NO signing session.
    pub fn sign(&self, req: BankIdNoSignRequest) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.post("/v3/bankid-no/sign", &req)
    }

    /// Start a BankID NO age verification session.
    pub fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/bankid-no/age-verification", &req)
    }

    /// Poll the current status of a BankID NO authentication session.
    pub fn auth_status(&self, id: &str) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.get(&format!("/v3/bankid-no/auth/{}", id))
    }

    /// Poll the current status of a BankID NO signing session.
    pub fn sign_status(&self, id: &str) -> Result<BankIdNoStatus, IdkollenError> {
        self.0.get(&format!("/v3/bankid-no/sign/{}", id))
    }

    /// Poll the current status of a BankID NO age verification session.
    pub fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0
            .get(&format!("/v3/bankid-no/age-verification/{}", id))
    }

    /// Cancel a BankID NO authentication session.
    pub fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/bankid-no/auth/{}", id))
    }

    /// Cancel a BankID NO signing session.
    pub fn cancel_sign(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/bankid-no/sign/{}", id))
    }

    /// Cancel a BankID NO age verification session.
    pub fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0
            .delete(&format!("/v3/bankid-no/age-verification/{}", id))
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub fn wait_for_auth(&self, id: &str, opts: PollOptions) -> Result<BankIdNoStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id)?;

            match status {
                BankIdNoStatus::Pending(_) => {
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
    pub fn wait_for_sign(&self, id: &str, opts: PollOptions) -> Result<BankIdNoStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.sign_status(id)?;

            match status {
                BankIdNoStatus::Pending(_) => {
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
