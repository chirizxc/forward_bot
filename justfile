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

@docker-build VERSION="latest":
    docker build -t forward_bot:{{VERSION}} .

@docker-run:
    docker run --rm -it \
        -v $(pwd)/config.toml:/app/config.toml \
        --name forward_bot \
        forward_bot

@docker-stop:
    docker stop forward_bot

@docker-rm: docker-stop
    docker rm forward_bot

docker:
    @just docker-stop >/dev/null 2>&1 || true
    @just docker-rm >/dev/null 2>&1 || true
    @just docker-run

@docker-up:
    docker compose up

@docker-down:
    docker compose down

@docker-rmi VERSION="latest": docker-down
    docker rmi forward_bot:{{VERSION}}

docker-tag-to-locale USER VERSION="latest":
    docker tag {{USER}}/forward_bot:{{VERSION}} forward_bot:{{VERSION}}

docker-pull USER VERSION="latest":
    docker pull {{USER}}/forward_bot:{{VERSION}}
    @just docker-tag-to-locale {{USER}} {{VERSION}}

docker-tag-to-remote USER VERSION="latest":
    docker tag forward_bot:{{VERSION}} {{USER}}/forward_bot:{{VERSION}}

docker-push USER VERSION="latest":
    @just docker-build {{VERSION}}
    @just docker-tag-to-remote {{USER}} {{VERSION}}
    docker push {{USER}}/forward_bot:{{VERSION}}
