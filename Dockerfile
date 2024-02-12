# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# If you’re using stable, use this instead
# FROM rust:1.74-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /islands-arch-test
WORKDIR /islands-arch-test
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bullseye as runner

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /islands-arch-test/target/release/islands-arch-test /islands-arch-test/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /islands-arch-test/target/site /islands-arch-test/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /islands-arch-test/Cargo.toml /islands-arch-test/
WORKDIR /islands-arch-test

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
ENV DIRECTUSURL="https://directus-ts-dev-production.jabratech-core.zeet.app"
ENV JABRAKEY="jabrakey123"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/islands-arch-test/islands-arch-test"]