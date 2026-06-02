use crate::client::{IdkollenError, PollOptions, WaitError};
use crate::models::{VippsAuthRequest, VippsBackchannelAuthRequest, VippsStatus};
use std::time::Instant;

#[cfg(feature = "async")]
use crate::client::IdkollenClient;

#[cfg(feature = "async")]
/// Vipps operations.
pub struct VippsEndpoint<'a>(pub(crate) &'a IdkollenClient);

#[cfg(feature = "async")]
impl VippsEndpoint<'_> {
    /// Start a Vipps authentication session.
    pub async fn auth(&self, req: VippsAuthRequest) -> Result<VippsStatus, IdkollenError> {
        self.0.post("/v3/vipps/auth", &req).await
    }

    /// Start a Vipps backchannel authentication session.
    pub async fn backchannel_auth(
        &self,
        req: VippsBackchannelAuthRequest,
    ) -> Result<VippsStatus, IdkollenError> {
        self.0.post("/v3/vipps/backchannel/auth", &req).await
    }

    /// Poll the current status of a Vipps authentication session.
    pub async fn auth_status(&self, id: &str) -> Result<VippsStatus, IdkollenError> {
        self.0.get(&format!("/v3/vipps/auth/{}", id)).await
    }

    /// Cancel a Vipps authentication session.
    pub async fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/vipps/auth/{}", id)).await
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub async fn wait_for_auth(
        &self,
        id: &str,
        opts: PollOptions,
    ) -> Result<VippsStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id).await?;

            match status {
                VippsStatus::Pending(_) => {
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
/// Vipps operations (blocking).
pub struct VippsBlockingEndpoint<'a>(pub(crate) &'a IdkollenBlockingClient);

#[cfg(feature = "blocking")]
impl VippsBlockingEndpoint<'_> {
    /// Start a Vipps authentication session.
    pub fn auth(&self, req: VippsAuthRequest) -> Result<VippsStatus, IdkollenError> {
        self.0.post("/v3/vipps/auth", &req)
    }

    /// Start a Vipps backchannel authentication session.
    pub fn backchannel_auth(
        &self,
        req: VippsBackchannelAuthRequest,
    ) -> Result<VippsStatus, IdkollenError> {
        self.0.post("/v3/vipps/backchannel/auth", &req)
    }

    /// Poll the current status of a Vipps authentication session.
    pub fn auth_status(&self, id: &str) -> Result<VippsStatus, IdkollenError> {
        self.0.get(&format!("/v3/vipps/auth/{}", id))
    }

    /// Cancel a Vipps authentication session.
    pub fn cancel_auth(&self, id: &str) -> Result<(), IdkollenError> {
        self.0.delete(&format!("/v3/vipps/auth/{}", id))
    }

    /// Poll until the authentication session reaches a terminal state or the timeout elapses.
    pub fn wait_for_auth(&self, id: &str, opts: PollOptions) -> Result<VippsStatus, WaitError> {
        let deadline = Instant::now() + opts.timeout;

        loop {
            let status = self.auth_status(id)?;

            match status {
                VippsStatus::Pending(_) => {
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
