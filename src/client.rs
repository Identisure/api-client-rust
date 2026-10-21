use reqwest::Method;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
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
    timeout: Duration,
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
            timeout: Duration::from_secs(30),
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
        let url: String = url.into();
        self.base_url = Some(url.trim_end_matches('/').to_owned());
        self
    }

    /// Override the `User-Agent` header sent with every request.
    #[inline]
    #[must_use]
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    /// Timeout for each HTTP request, including reading the response body. Must be positive. Default: 30 s.
    #[inline]
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
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
        let http = self
            .http_client
            .map(Ok)
            .unwrap_or_else(|| reqwest::Client::builder().build())?;

        Ok(IdkollenClient {
            http,
            base_url,
            client_id: self.client_id,
            client_secret: self.client_secret,
            user_agent: self.user_agent,
            timeout: self.timeout,
        })
    }

    #[cfg(feature = "blocking")]
    pub fn build_blocking(self) -> Result<IdkollenBlockingClient, IdkollenError> {
        let base_url = self
            .base_url
            .unwrap_or_else(|| self.environment.base_url().to_owned());
        let http = self
            .blocking_http_client
            .map(Ok)
            .unwrap_or_else(|| reqwest::blocking::Client::builder().build())?;

        Ok(IdkollenBlockingClient {
            http,
            base_url,
            client_id: self.client_id,
            client_secret: self.client_secret,
            user_agent: self.user_agent,
            timeout: self.timeout,
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
    pub(crate) timeout: Duration,
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

    fn request(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        self.http
            .request(method, format!("{}{}", self.base_url, path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .timeout(self.timeout)
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, IdkollenError> {
        let resp = self.request(Method::GET, path).send().await?;

        parse_response(resp).await
    }

    pub(crate) async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, IdkollenError> {
        let resp = self.request(Method::POST, path).json(body).send().await?;

        parse_response(resp).await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<(), IdkollenError> {
        let resp = self.request(Method::DELETE, path).send().await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let message = api_error_message(resp.text().await.unwrap_or_default());

            Err(IdkollenError::Api { status, message })
        }
    }

    pub(crate) async fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T, IdkollenError> {
        let resp = self
            .request(Method::POST, path)
            .multipart(form)
            .send()
            .await?;

        parse_response(resp).await
    }

    pub(crate) async fn get_bytes(&self, path: &str) -> Result<Vec<u8>, IdkollenError> {
        let resp = self.request(Method::GET, path).send().await?;

        if resp.status().is_success() {
            Ok(resp.bytes().await?.to_vec())
        } else {
            let status = resp.status().as_u16();
            let message = api_error_message(resp.text().await.unwrap_or_default());

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
        let message = api_error_message(resp.text().await.unwrap_or_default());

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
    pub(crate) timeout: Duration,
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

    fn request(&self, method: Method, path: &str) -> reqwest::blocking::RequestBuilder {
        self.http
            .request(method, format!("{}{}", self.base_url, path))
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .header(USER_AGENT, &self.user_agent)
            .timeout(self.timeout)
    }

    pub(crate) fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, IdkollenError> {
        let resp = self.request(Method::GET, path).send()?;

        parse_blocking_response(resp)
    }

    pub(crate) fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, IdkollenError> {
        let resp = self.request(Method::POST, path).json(body).send()?;

        parse_blocking_response(resp)
    }

    pub(crate) fn delete(&self, path: &str) -> Result<(), IdkollenError> {
        let resp = self.request(Method::DELETE, path).send()?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status().as_u16();
            let message = api_error_message(resp.text().unwrap_or_default());

            Err(IdkollenError::Api { status, message })
        }
    }

    pub(crate) fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::blocking::multipart::Form,
    ) -> Result<T, IdkollenError> {
        let resp = self.request(Method::POST, path).multipart(form).send()?;

        parse_blocking_response(resp)
    }

    pub(crate) fn get_bytes(&self, path: &str) -> Result<Vec<u8>, IdkollenError> {
        let resp = self.request(Method::GET, path).send()?;

        if resp.status().is_success() {
            Ok(resp.bytes()?.to_vec())
        } else {
            let status = resp.status().as_u16();
            let message = api_error_message(resp.text().unwrap_or_default());

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
        let message = api_error_message(resp.text().unwrap_or_default());

        Err(IdkollenError::Api { status, message })
    }
}

#[cfg(all(test, feature = "blocking"))]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn request_timeout_returns_http_error() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            let (_stream, _) = listener.accept().unwrap();
            thread::sleep(Duration::from_secs(2));
        });

        let client = IdkollenClientBuilder::new("id", "secret")
            .base_url(format!("http://{addr}"))
            .timeout(Duration::from_millis(100))
            .build_blocking()
            .unwrap();
        let result = client.get::<serde_json::Value>("/v3/slow");

        assert!(matches!(result, Err(IdkollenError::Http(e)) if e.is_timeout()));
    }
}

fn api_error_message(body: String) -> String {
    #[derive(Deserialize)]
    struct ErrorBody {
        message: String,
    }

    serde_json::from_str::<ErrorBody>(&body).map_or(body, |e| e.message)
}
