# IDkollen Rust Client

API client for the [IDkollen](https://developers.idkollen.se) REST API.

## Features

- **`async`** (default) — async/await via `tokio`
- **`blocking`** - synchronous API via `reqwest::blocking`

## Usage

```toml
[dependencies]
idkollen-client = { version = "0.1", features = ["async"] }
```

```rust
use idkollen_client::{IdkollenClientBuilder, Environment};
use idkollen_client::models::{BankIdSeAuthRequest, PollOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = IdkollenClientBuilder::new("client_id", "client_secret")
        .environment(Environment::Staging)
        .build()?;

    let session = client
        .bankid_se()
        .auth(BankIdSeAuthRequest::new())
        .await?;

    let result = client
        .bankid_se()
        .wait_for_auth(&session.id(), PollOptions::default())
        .await?;

    println!("{result:#?}");
    Ok(())
}
```
