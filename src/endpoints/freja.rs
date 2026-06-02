use crate::client::{IdkollenError, PollOptions, WaitError};
use crate::models::{AgeVerificationRequest, AgeVerificationStatus};
use crate::models::{
    FrejaAuthRequest, FrejaBackchannelAuthRequest, FrejaBackchannelSignRequest, FrejaSignRequest,
    FrejaStatus,
};
use std::time::Instant;

#[cfg(feature = "async")]
use crate::client::IdkollenClient;

#[cfg(feature = "async")]
/// Freja eID operations.
pub struct FrejaEndpoint<'a>(pub(crate) &'a IdkollenClient);

#[cfg(feature = "async")]
impl FrejaEndpoint<'_> {
    /// Start a Freja eID authentication session.
    pub async fn auth(&self, req: FrejaAuthRequest) -> Result<FrejaStatus, IdkollenError> {
        self.0.post("/v3/freja/auth", &req).await
    }

    /// Start a Freja eID backchannel authentication session.
    pub async fn backchannel_auth(
        &self,
        req: FrejaBackchannelAuthRequest,
    ) -> Result<FrejaStatus, IdkollenError> {
        self.0.post("/v3/freja/backchannel/auth", &req).await
    }

    /// Start a Freja eID signing session.
    pub async fn sign(&self, req: FrejaSignRequest) -> Result<FrejaStatus, IdkollenError> {
        self.0.post("/v3/freja/sign", &req).await
    }

    /// Start a Freja eID backchannel signing session.
    pub async fn backchannel_sign(
        &self,
        req: FrejaBackchannelSignRequest,
    ) -> Result<FrejaStatus, IdkollenError> {
        self.0.post("/v3/freja/backchannel/sign", &req).await
    }

    /// Start a Freja eID age verification session.
    pub async fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/freja/age-verification", &req).await
    }

    /// Poll the current status of a Freja eID authentication session.
    pub async fn auth_status(&self, id: &str) -> Result<FrejaStatus, IdkollenError> {
        self.0.get(&format!("/v3/freja/auth/{}", id)).await
    }

    /// Poll the current status of a Freja eID signing session.
    pub async fn sign_status(&self, id: &str) -> Result<FrejaStatus, IdkollenError> {
        self.0.get(&format!("/v3/freja/sign/{}", id)).await
    }

    /// Poll the current status of a Freja eID age verification session.
    pub async fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0
            .get(&format!("/v3/freja/age-verification/{}", id))
            .await
    }

    /// Cancel a Freja eID authentication session.
    pub async fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/freja/auth/{}", id)).await
    }

    /// Cancel a Freja eID signing session.
    pub async fn cancel_sign(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/freja/sign/{}", id)).await
    }

    /// Cancel a Freja eID age verification session.
    pub async fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0
            .delete(&format!("/v3/freja/age-verification/{}", id))
            .await
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub async fn wait_for_auth(
        &self,
        id: &str,
        opts: PollOptions,
    ) -> Result<FrejaStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id).await?;

            match status {
                FrejaStatus::Pending(_) => {
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
    ) -> Result<FrejaStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.sign_status(id).await?;

            match status {
                FrejaStatus::Pending(_) => {
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
/// Freja eID operations (blocking).
pub struct FrejaBlockingEndpoint<'a>(pub(crate) &'a IdkollenBlockingClient);

#[cfg(feature = "blocking")]
impl FrejaBlockingEndpoint<'_> {
    /// Start a Freja eID authentication session.
    pub fn auth(&self, req: FrejaAuthRequest) -> Result<FrejaStatus, IdkollenError> {
        self.0.post("/v3/freja/auth", &req)
    }

    /// Start a Freja eID backchannel authentication session.
    pub fn backchannel_auth(
        &self,
        req: FrejaBackchannelAuthRequest,
    ) -> Result<FrejaStatus, IdkollenError> {
        self.0.post("/v3/freja/backchannel/auth", &req)
    }

    /// Start a Freja eID signing session.
    pub fn sign(&self, req: FrejaSignRequest) -> Result<FrejaStatus, IdkollenError> {
        self.0.post("/v3/freja/sign", &req)
    }

    /// Start a Freja eID backchannel signing session.
    pub fn backchannel_sign(
        &self,
        req: FrejaBackchannelSignRequest,
    ) -> Result<FrejaStatus, IdkollenError> {
        self.0.post("/v3/freja/backchannel/sign", &req)
    }

    /// Start a Freja eID age verification session.
    pub fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/freja/age-verification", &req)
    }

    /// Poll the current status of a Freja eID authentication session.
    pub fn auth_status(&self, id: &str) -> Result<FrejaStatus, IdkollenError> {
        self.0.get(&format!("/v3/freja/auth/{}", id))
    }

    /// Poll the current status of a Freja eID signing session.
    pub fn sign_status(&self, id: &str) -> Result<FrejaStatus, IdkollenError> {
        self.0.get(&format!("/v3/freja/sign/{}", id))
    }

    /// Poll the current status of a Freja eID age verification session.
    pub fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.get(&format!("/v3/freja/age-verification/{}", id))
    }

    /// Cancel a Freja eID authentication session.
    pub fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/freja/auth/{}", id))
    }

    /// Cancel a Freja eID signing session.
    pub fn cancel_sign(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/freja/sign/{}", id))
    }

    /// Cancel a Freja eID age verification session.
    pub fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/freja/age-verification/{}", id))
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub fn wait_for_auth(&self, id: &str, opts: PollOptions) -> Result<FrejaStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id)?;

            match status {
                FrejaStatus::Pending(_) => {
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
    pub fn wait_for_sign(&self, id: &str, opts: PollOptions) -> Result<FrejaStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.sign_status(id)?;

            match status {
                FrejaStatus::Pending(_) => {
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
