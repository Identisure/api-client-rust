use crate::client::{IdkollenError, PollOptions, WaitError};
use crate::models::{AgeVerificationRequest, AgeVerificationStatus};
use crate::models::{MitIdAuthRequest, MitIdBackchannelAuthRequest, MitIdSignRequest, MitIdStatus};
use std::time::Instant;

#[cfg(feature = "async")]
use crate::client::IdkollenClient;

#[cfg(feature = "async")]
/// MitID operations.
pub struct MitIdEndpoint<'a>(pub(crate) &'a IdkollenClient);

#[cfg(feature = "async")]
impl MitIdEndpoint<'_> {
    /// Start a MitID authentication session.
    pub async fn auth(&self, req: MitIdAuthRequest) -> Result<MitIdStatus, IdkollenError> {
        self.0.post("/v3/mitid/auth", &req).await
    }

    /// Start a MitID backchannel authentication session.
    pub async fn backchannel_auth(
        &self,
        req: MitIdBackchannelAuthRequest,
    ) -> Result<MitIdStatus, IdkollenError> {
        self.0.post("/v3/mitid/backchannel/auth", &req).await
    }

    /// Start a MitID signing session.
    pub async fn sign(&self, req: MitIdSignRequest) -> Result<MitIdStatus, IdkollenError> {
        self.0.post("/v3/mitid/sign", &req).await
    }

    /// Start a MitID age verification session.
    pub async fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/mitid/age-verification", &req).await
    }

    /// Poll the current status of a MitID authentication session.
    pub async fn auth_status(&self, id: &str) -> Result<MitIdStatus, IdkollenError> {
        self.0.get(&format!("/v3/mitid/auth/{}", id)).await
    }

    /// Poll the current status of a MitID signing session.
    pub async fn sign_status(&self, id: &str) -> Result<MitIdStatus, IdkollenError> {
        self.0.get(&format!("/v3/mitid/sign/{}", id)).await
    }

    /// Poll the current status of a MitID age verification session.
    pub async fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0
            .get(&format!("/v3/mitid/age-verification/{}", id))
            .await
    }

    /// Cancel a MitID authentication session.
    pub async fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/mitid/auth/{}", id)).await
    }

    /// Cancel a MitID signing session.
    pub async fn cancel_sign(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/mitid/sign/{}", id)).await
    }

    /// Cancel a MitID age verification session.
    pub async fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0
            .delete(&format!("/v3/mitid/age-verification/{}", id))
            .await
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub async fn wait_for_auth(
        &self,
        id: &str,
        opts: PollOptions,
    ) -> Result<MitIdStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id).await?;

            match status {
                MitIdStatus::Pending(_) => {
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
    ) -> Result<MitIdStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;
        loop {
            let status = self.sign_status(id).await?;
            match status {
                MitIdStatus::Pending(_) => {
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
/// MitID operations (blocking).
pub struct MitIdBlockingEndpoint<'a>(pub(crate) &'a IdkollenBlockingClient);

#[cfg(feature = "blocking")]
impl MitIdBlockingEndpoint<'_> {
    /// Start a MitID authentication session.
    pub fn auth(&self, req: MitIdAuthRequest) -> Result<MitIdStatus, IdkollenError> {
        self.0.post("/v3/mitid/auth", &req)
    }

    /// Start a MitID backchannel authentication session.
    pub fn backchannel_auth(
        &self,
        req: MitIdBackchannelAuthRequest,
    ) -> Result<MitIdStatus, IdkollenError> {
        self.0.post("/v3/mitid/backchannel/auth", &req)
    }

    /// Start a MitID signing session.
    pub fn sign(&self, req: MitIdSignRequest) -> Result<MitIdStatus, IdkollenError> {
        self.0.post("/v3/mitid/sign", &req)
    }

    /// Start a MitID age verification session.
    pub fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/mitid/age-verification", &req)
    }

    /// Poll the current status of a MitID authentication session.
    pub fn auth_status(&self, id: &str) -> Result<MitIdStatus, IdkollenError> {
        self.0.get(&format!("/v3/mitid/auth/{}", id))
    }

    /// Poll the current status of a MitID signing session.
    pub fn sign_status(&self, id: &str) -> Result<MitIdStatus, IdkollenError> {
        self.0.get(&format!("/v3/mitid/sign/{}", id))
    }

    /// Poll the current status of a MitID age verification session.
    pub fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.get(&format!("/v3/mitid/age-verification/{}", id))
    }

    /// Cancel a MitID authentication session.
    pub fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/mitid/auth/{}", id))
    }

    /// Cancel a MitID signing session.
    pub fn cancel_sign(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/mitid/sign/{}", id))
    }

    /// Cancel a MitID age verification session.
    pub fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/mitid/age-verification/{}", id))
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub fn wait_for_auth(&self, id: &str, opts: PollOptions) -> Result<MitIdStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id)?;

            match status {
                MitIdStatus::Pending(_) => {
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
    pub fn wait_for_sign(&self, id: &str, opts: PollOptions) -> Result<MitIdStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.sign_status(id)?;

            match status {
                MitIdStatus::Pending(_) => {
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
