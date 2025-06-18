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

@docker-build:
    docker build -t forward_bot .

@docker-run:
    docker run --rm -it \
        -v $(pwd)/config.toml:/app/config.toml \
        --name forward_bot \
        forward_bot

@docker-stop:
    docker stop forward_bot

@docker-rm:
    docker rm forward_bot

@docker:
    just docker-stop >/dev/null 2>&1 || true
    just docker-rm >/dev/null 2>&1 || true
    just docker-run
