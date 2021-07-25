# We use the lastest Rust stale release as base image
FROM rust:1.53.0

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

# Env for production build on docker
ENV APP_ENVIRONMENT production

# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/email_newsletter"]