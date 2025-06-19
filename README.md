# 🦀 forward_bot

## Running via Cargo

From the project directory, you can build and run **forward_bot** directly with Cargo using the included `just` tasks:

- `just run` — build and run the bot in **debug** mode (development build)  
- `just run-release` — build and run the bot in **release** mode (optimized build)  
- `CONFIG=config-another.toml just run` — run with an alternate configuration file (`config-another.toml` instead of the default)

## Running via Docker

Alternatively, you can run **forward_bot** as a Docker container.

### Basic Docker Workflow

- `just docker-build [VERSION=latest]` — build the Docker image locally. By default, tags as `forward-bot:latest`, or `forward-bot:<VERSION>` if specified  
- `just docker-run [VERSION=latest]` — run the Docker container from the built image. By default, uses `forward-bot:latest`, or `forward-bot:<VERSION>` if specified.
- `just docker-stop` — stop the running container  
- `just docker-rm` — remove the stopped container  
- `just docker [VERSION=latest]` — convenience command that stops, removes, then runs the container
- `just docker-rmi [VERSION=latest]` — remove the Docker image. By default, removes `forward-bot:latest`, or `forward-bot:<VERSION>` if specified

### Docker Compose

You can also use Docker Compose to manage the bot and its dependencies without needing to manually build and run the container each time:

- `just docker-up` — start the service
- `just docker-down` — down the service

## Docker Image Management

Use these commands to manage Docker images:

- `just docker-pull <username> [VERSION=latest]` — pull the `<username>/forward-bot:<VERSION>` image from Docker Hub and tag it locally
- `just docker-push <username> [VERSION=latest]` — build, tag, and push the image to `<username>/forward-bot:<VERSION>` on Docker Hub

## Formatting and Linting

For code style and lint checks, run:

- `just fmt` — format the source code (runs `cargo fmt`)  
- `just clippy` — run the Rust linter (runs `cargo clippy`) to catch common issues
