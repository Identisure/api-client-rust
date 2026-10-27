<img align="left" height="100" style="margin-right: 10px;" src="./img/icon.png">

# Identisure Rust Client

[![Crate][crate-image]][crate-url]
[![Downloads][downloads-image]][downloads-url]
[![Documentation][docs-image]][docs-url]
[![MIT license][license-image]][license-url]

[crate-image]: https://img.shields.io/crates/v/identisure-client?style=flat-square
[crate-url]: https://crates.io/crates/identisure-client
[downloads-image]: https://img.shields.io/crates/d/identisure-client?style=flat-square
[downloads-url]: https://crates.io/crates/identisure-client
[docs-image]: https://img.shields.io/docsrs/identisure-client?style=flat-square
[docs-url]: https://docs.rs/identisure-client
[license-image]: https://img.shields.io/badge/license-MIT-blue?style=flat-square
[license-url]: https://opensource.org/license/mit

API client for the [Identisure](https://developers.identisure.com) REST API.

## Requirements

Rust 1.87 or later.

## Supported providers

- **BankID SE**: auth, sign, phone auth and sign, QR verify, age verification
- **BankID NO**: auth, backchannel auth, sign, age verification
- **Freja eID**: auth, backchannel auth, sign, backchannel sign, age verification
- **MitID**: auth, backchannel auth, sign, age verification
- **FTN** (Finnish Trust Network): auth, age verification
- **Vipps MobilePay**: auth, backchannel auth
- **Documents**: upload and download documents for signing

## Installation

```sh
cargo add identisure-client
```

### Features

* **`async`** (default) — async/await via `tokio`
* **`blocking`** - synchronous API via `reqwest::blocking`

## Usage

```toml
[dependencies]
identisure-client = { version = "0.1", features = ["async"] }
```

```rust
use identisure_client::models::{BankIdSeAuthRequest, BankIdSeStatus};
use identisure_client::{Environment, IdentisureClientBuilder, PollOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = IdentisureClientBuilder::new("client_id", "client_secret")
        .environment(Environment::Staging)
        .build()?;

    let BankIdSeStatus::Pending(session) =
        client.bankid_se().auth(BankIdSeAuthRequest::new()).await?
    else {
        return Err("Expected a pending session".into());
    };

    let result = client
        .bankid_se()
        .wait_for_auth(&session.id, PollOptions::default())
        .await?;

    println!("{result:#?}");
    Ok(())
}
```

### Verifying callbacks

```rust
let valid = identisure_client::verify_callback_signature(&raw_body, signature_header, hmac_secret);
```

## Development

#### Prerequisites

* [Rust Stable 1.87+](https://rust-lang.org/tools/install/)
* [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

#### Clone

The test fixtures live in a git submodule. Clone with:

```sh
$ git clone --recurse-submodules https://github.com/identisure/api-client-rust.git
```

Or, in an existing clone, run `git submodule update --init`.

#### Build

To compile the project, simply issue the following command:

```sh
$ cargo build --all-features
```

#### Test

##### Linting

This project uses [rustfmt](https://github.com/rust-lang/rustfmt) for formatting and 
[clippy](https://github.com/rust-lang/rust-clippy) for linting. Run them with:

```sh
$ cargo fmt --check
$ cargo clippy --all-features
```

##### Unit/Integration/Doc Testing 

```sh
$ cargo test --all-features
```

#### Release

Set the version in `Cargo.toml` and commit. Then push a tag:

```sh
$ git tag v0.1.0
$ git push origin v0.1.0
```

The release workflow checks that the tag matches the version, runs CI, publishes to crates.io and creates a GitHub release.

## Other languages

[C#](https://github.com/identisure/api-client-csharp), [Dart](https://github.com/identisure/api-client-dart), [Go](https://github.com/identisure/api-client-go), [TypeScript/JavaScript](https://github.com/identisure/api-client-js), [Java/Kotlin](https://github.com/identisure/api-client-jvm), [PHP](https://github.com/identisure/api-client-php), [Python](https://github.com/identisure/api-client-py), [Ruby](https://github.com/identisure/api-client-ruby), [Swift](https://github.com/identisure/api-client-swift)
