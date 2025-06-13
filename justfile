host := `uname -a`

help:
    just -l

clippy:
    cargo clippy --all --all-features -- -W clippy::pedantic

fmt:
    cargo fmt --all -- --check

@build:
    cargo build --all-features

@build-release:
    cargo build --release --all-features

@run: build
    cargo run

@run-release: build-release
    cargo run --release
