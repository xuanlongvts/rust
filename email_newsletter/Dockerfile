# ---- Cache stage
FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 as planner
WORKDIR /app
COPY . .

# Compute a lock-file file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

# ---- Build stage
# We use the lastest Rust stale release as base image
FROM rust:1.53.0 AS builder

# Let's switch out working directory to `app` (equivalent to `cd app`) 
# The `app` folder will be created for us by Docker in case it doest not exist already.
WORKDIR /app

# Copy all files from our working environment to our Docker image
COPY . .

# Set sqlx offline is true for docker get env local not network
ENV SQLX_OFFLINE true

# Let's build our binary!
# We'll use the release profile to make it fast
RUN cargo build --release

# ---- Runtime stage
FROM debian:buster-slim AS runtime

WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/email_newsletter email_newsletter

COPY configuration configuration

# Env for production build on docker
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./email_newsletter"]