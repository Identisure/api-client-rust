use crate::client::{IdkollenError, PollOptions, WaitError};
use crate::models::{AgeVerificationRequest, AgeVerificationStatus};
use crate::models::{FtnAuthRequest, FtnStatus};
use std::time::Instant;

#[cfg(feature = "async")]
use crate::client::IdkollenClient;

#[cfg(feature = "async")]
/// FTN operations.
pub struct FtnEndpoint<'a>(pub(crate) &'a IdkollenClient);

#[cfg(feature = "async")]
impl FtnEndpoint<'_> {
    /// Start a FTN authentication session.
    pub async fn auth(&self, req: FtnAuthRequest) -> Result<FtnStatus, IdkollenError> {
        self.0.post("/v3/ftn/auth", &req).await
    }

    /// Start a FTN age verification session.
    pub async fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/ftn/age-verification", &req).await
    }

    /// Poll the current status of a FTN authentication session.
    pub async fn auth_status(&self, id: &str) -> Result<FtnStatus, IdkollenError> {
        self.0.get(&format!("/v3/ftn/auth/{}", id)).await
    }

    /// Poll the current status of a FTN age verification session.
    pub async fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0
            .get(&format!("/v3/ftn/age-verification/{}", id))
            .await
    }

    /// Cancel a FTN authentication session.
    pub async fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/ftn/auth/{}", id)).await
    }

    /// Cancel a FTN age verification session.
    pub async fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0
            .delete(&format!("/v3/ftn/age-verification/{}", id))
            .await
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub async fn wait_for_auth(&self, id: &str, opts: PollOptions) -> Result<FtnStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id).await?;

            match status {
                FtnStatus::Pending(_) => {
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
/// FTN operations (blocking).
pub struct FtnBlockingEndpoint<'a>(pub(crate) &'a IdkollenBlockingClient);

#[cfg(feature = "blocking")]
impl FtnBlockingEndpoint<'_> {
    /// Start a FTN authentication session.
    pub fn auth(&self, req: FtnAuthRequest) -> Result<FtnStatus, IdkollenError> {
        self.0.post("/v3/ftn/auth", &req)
    }

    /// Start a FTN age verification session.
    pub fn age_verification(
        &self,
        req: AgeVerificationRequest,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.post("/v3/ftn/age-verification", &req)
    }

    /// Poll the current status of a FTN authentication session.
    pub fn auth_status(&self, id: &str) -> Result<FtnStatus, IdkollenError> {
        self.0.get(&format!("/v3/ftn/auth/{}", id))
    }

    /// Poll the current status of a FTN age verification session.
    pub fn age_verification_status(
        &self,
        id: &str,
    ) -> Result<AgeVerificationStatus, IdkollenError> {
        self.0.get(&format!("/v3/ftn/age-verification/{}", id))
    }

    /// Cancel a FTN authentication session.
    pub fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/ftn/auth/{}", id))
    }

    /// Cancel a FTN age verification session.
    pub fn cancel_age_verification(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/ftn/age-verification/{}", id))
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub fn wait_for_auth(&self, id: &str, opts: PollOptions) -> Result<FtnStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id)?;

            match status {
                FtnStatus::Pending(_) => {
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
