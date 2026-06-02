use reqwest::header::USER_AGENT;
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;
use thiserror::Error;

#[cfg(feature = "async")]
use crate::endpoints::{
    BankIdNoEndpoint, BankIdSeEndpoint, DocumentEndpoint, FrejaEndpoint, FtnEndpoint,
    MitIdEndpoint, VippsEndpoint,
};

#[cfg(feature = "blocking")]
use crate::endpoints::{
    BankIdNoBlockingEndpoint, BankIdSeBlockingEndpoint, DocumentBlockingEndpoint,
    FrejaBlockingEndpoint, FtnBlockingEndpoint, MitIdBlockingEndpoint, VippsBlockingEndpoint,
};

/// Errors returned by all client operations.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum IdkollenError {
    /// Network-level error (connection refused, TLS failure, etc.).
    #[error("HTTP error: {0}")]
    Http(
        #[from]
        #[source]
        reqwest::Error,
    ),
    /// The server returned a non-2xx response.
    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },
    /// The server returned an unexpected JSON shape.
    #[error("JSON error: {0}")]
    Deserialization(
        #[from]
        #[source]
        serde_path_to_error::Error<serde_json::Error>,
    ),
}

/// Error returned by `wait_for_*` polling helpers.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum WaitError {
    /// Polling exceeded the configured timeout without reaching a terminal state.
    #[error("Poll timed out without reaching a terminal state")]
    Timeout,
    /// An underlying network or API error occurred during polling.
    #[error(transparent)]
    Client(#[from] IdkollenError),
}

/// Pre-configured API base URL selection.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Environment {
    /// `https://api.idkollen.se`
    Production,
    /// `https://stgapi.idkollen.se`
    Staging,
}

impl Environment {
    #[inline]
    #[must_use]
    fn base_url(&self) -> &'static str {
        match self {
            Self::Production => "https://api.idkollen.se",
            Self::Staging => "https://stgapi.idkollen.se",
        }
    }
}

/// Options for the high-level `wait_for_*` polling helpers.
#[derive(Debug, Clone)]
pub struct PollOptions {
    /// How long to sleep between status polls. Default: 2 s.
    pub interval: Duration,
    /// Maximum total time to wait before returning [`WaitError::Timeout`]. Default: 300 s.
    pub timeout: Duration,
}

impl Default for PollOptions {
    #[inline]
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(2),
            timeout: Duration::from_secs(300),
        }
    }
}

/// Builder for constructing an [`IdkollenClient`] or [`IdkollenBlockingClient`].
pub struct IdkollenClientBuilder {
    environment: Environment,
    base_url: Option<String>,
    client_id: String,
    client_secret: String,
    user_agent: String,
    #[cfg(feature = "async")]
    http_client: Option<reqwest::Client>,
    #[cfg(feature = "blocking")]
    blocking_http_client: Option<reqwest::blocking::Client>,
}

impl IdkollenClientBuilder {
    #[must_use]
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            environment: Environment::Production,
            base_url: None,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            user_agent: format!("idkollen-client-rs/{}", env!("CARGO_PKG_VERSION")),
            #[cfg(feature = "async")]
            http_client: None,
            #[cfg(feature = "blocking")]
            blocking_http_client: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn environment(mut self, env: Environment) -> Self {
        self.environment = env;
        self
    }

    #[inline]
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Override the `User-Agent` header sent with every request.
    #[inline]
    #[must_use]
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    #[cfg(feature = "async")]
    #[inline]
    #[must_use]
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    #[cfg(feature = "blocking")]
    #[inline]
    #[must_use]
    pub fn blocking_http_client(mut self, client: reqwest::blocking::Client) -> Self {
        self.blocking_http_client = Some(client);
        self
    }

    #[cfg(feature = "async")]
    pub fn build(self) -> Result<IdkollenClient, IdkollenError> {
        let base_url = self
            .base_url
            .unwrap_or_else(|| self.environment.base_url().to_owned());
        let http = self.http_client.map(Ok).unwrap_or_else(|| {
            reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
        })?;

        Ok(IdkollenClient {
            http,
            base_url,
            client_id: self.client_id,
            client_secret: self.client_secret,
            user_agent: self.user_agent,
        })
    }

    #[cfg(feature = "blocking")]
    pub fn build_blocking(self) -> Result<IdkollenBlockingClient, IdkollenError> {
        let base_url = self
            .base_url
            .unwrap_or_else(|| self.environment.base_url().to_owned());
        let http = self.blocking_http_client.map(Ok).unwrap_or_else(|| {
            reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
        })?;

        Ok(IdkollenBlockingClient {
            http,
            base_url,
            client_id: self.client_id,
            client_secret: self.client_secret,
            user_agent: self.user_agent,
        })
    }
}

#[cfg(feature = "async")]
pub struct IdkollenClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) user_agent: String,
}

#[cfg(feature = "async")]
impl IdkollenClient {
    #[inline]
    #[must_use]
    pub fn bankid_se(&self) -> BankIdSeEndpoint<'_> {
        BankIdSeEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn bankid_no(&self) -> BankIdNoEndpoint<'_> {
        BankIdNoEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn freja(&self) -> FrejaEndpoint<'_> {
        FrejaEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn mitid(&self) -> MitIdEndpoint<'_> {
        MitIdEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn ftn(&self) -> FtnEndpoint<'_> {
        FtnEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn vipps(&self) -> VippsEndpoint<'_> {
        VippsEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn document(&self) -> DocumentEndpoint<'_> {
        DocumentEndpoint(self)
    }

    #[inline]
    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, IdkollenError> {
        let resp = self
            .http
            .get(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .send()
            .await?;

        parse_response(resp).await
    }

    pub(crate) async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, IdkollenError> {
        let resp = self
            .http
            .post(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .json(body)
            .send()
            .await?;

        parse_response(resp).await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<(), IdkollenError> {
        let resp = self
            .http
            .delete(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .send()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();

            Err(IdkollenError::Api { status, message })
        }
    }

    pub(crate) async fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T, IdkollenError> {
        let resp = self
            .http
            .post(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .multipart(form)
            .send()
            .await?;

        parse_response(resp).await
    }

    pub(crate) async fn get_bytes(&self, path: &str) -> Result<Vec<u8>, IdkollenError> {
        let resp = self
            .http
            .get(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .send()
            .await?;

        if resp.status().is_success() {
            Ok(resp.bytes().await?.to_vec())
        } else {
            let status = resp.status().as_u16();
            let message = resp.text().await.unwrap_or_default();

            Err(IdkollenError::Api { status, message })
        }
    }
}

#[cfg(feature = "async")]
async fn parse_response<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, IdkollenError> {
    if resp.status().is_success() {
        let text = resp.text().await?;
        let deserializer = &mut serde_json::Deserializer::from_str(&text);

        Ok(serde_path_to_error::deserialize(deserializer)?)
    } else {
        let status = resp.status().as_u16();
        let message = resp.text().await.unwrap_or_default();

        Err(IdkollenError::Api { status, message })
    }
}

#[cfg(feature = "blocking")]
pub struct IdkollenBlockingClient {
    pub(crate) http: reqwest::blocking::Client,
    pub(crate) base_url: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) user_agent: String,
}

#[cfg(feature = "blocking")]
impl IdkollenBlockingClient {
    #[inline]
    #[must_use]
    pub fn bankid_se(&self) -> BankIdSeBlockingEndpoint<'_> {
        BankIdSeBlockingEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn bankid_no(&self) -> BankIdNoBlockingEndpoint<'_> {
        BankIdNoBlockingEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn freja(&self) -> FrejaBlockingEndpoint<'_> {
        FrejaBlockingEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn mitid(&self) -> MitIdBlockingEndpoint<'_> {
        MitIdBlockingEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn ftn(&self) -> FtnBlockingEndpoint<'_> {
        FtnBlockingEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn vipps(&self) -> VippsBlockingEndpoint<'_> {
        VippsBlockingEndpoint(self)
    }

    #[inline]
    #[must_use]
    pub fn document(&self) -> DocumentBlockingEndpoint<'_> {
        DocumentBlockingEndpoint(self)
    }

    #[inline]
    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub(crate) fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, IdkollenError> {
        let resp = self
            .http
            .get(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .send()?;

        parse_blocking_response(resp)
    }

    pub(crate) fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, IdkollenError> {
        let resp = self
            .http
            .post(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .json(body)
            .send()?;

        parse_blocking_response(resp)
    }

    pub(crate) fn delete(&self, path: &str) -> Result<(), IdkollenError> {
        let resp = self
            .http
            .delete(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .send()?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let message = resp.text().unwrap_or_default();

            Err(IdkollenError::Api { status, message })
        }
    }

    pub(crate) fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::blocking::multipart::Form,
    ) -> Result<T, IdkollenError> {
        let resp = self
            .http
            .post(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .multipart(form)
            .send()?;

        parse_blocking_response(resp)
    }

    pub(crate) fn get_bytes(&self, path: &str) -> Result<Vec<u8>, IdkollenError> {
        let resp = self
            .http
            .get(self.url(path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .send()?;

        if resp.status().is_success() {
            Ok(resp.bytes()?.to_vec())
        } else {
            let status = resp.status().as_u16();
            let message = resp.text().unwrap_or_default();

            Err(IdkollenError::Api { status, message })
        }
    }
}

#[cfg(feature = "blocking")]
fn parse_blocking_response<T: DeserializeOwned>(
    resp: reqwest::blocking::Response,
) -> Result<T, IdkollenError> {
    if resp.status().is_success() {
        let text = resp.text()?;
        let deserializer = &mut serde_json::Deserializer::from_str(&text);

        Ok(serde_path_to_error::deserialize(deserializer)?)
    } else {
        let status = resp.status().as_u16();
        let message = resp.text().unwrap_or_default();

        Err(IdkollenError::Api { status, message })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{BankIdSePhoneAuthRequest, CallInitiator, Pno};

    #[tokio::test]
    async fn test() {
        let client = IdkollenClientBuilder::new("494c2afa-fb68-4891-9b3b-8a0056771707", "123456")
            .environment(Environment::Staging)
            .build()
            .unwrap();

        let response = client
            .bankid_se()
            .phone_auth(BankIdSePhoneAuthRequest::new(
                Pno::parse("9012073731").unwrap(),
                CallInitiator::User,
            ))
            .await
            .unwrap();

        println!("{:#?}", response);
    }
}
