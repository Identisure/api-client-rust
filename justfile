build:
    cargo build --all-features

test:
    cargo test --all-features

check:
    cargo check --all-features
    cargo clippy --all-features --all-targets
    cargo fmt --check

publish:
    cargo publish

gen-doc:
    cargo doc --no-deps --all-features
    rm -rf docs
    cp -r target/doc docs
    echo '<meta http-equiv="refresh" content="0; url=identisure_client/">' > docs/index.html
