# TODO: this is currently broken, having issues w/ wanting to use nightly features
# but not being able to use nightly features in the cargo-chef image (doesn't build)
# FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
FROM lukemathwalker/cargo-chef:0.1.51-rust-1.66.1-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo +nightly chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo +nightly chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo +nightly build --release

# Let local directory be the dev environment
FROM rust:1 AS dev

# Copy the binary from the build stage
COPY --from=builder /app/target/release/leafc /usr/local/bin/leafc

# Run the binary
CMD ["leafc"]
