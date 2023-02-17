FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# Let local directory be the dev environment
FROM rust:1 AS dev

# Copy the binary from the build stage
COPY --from=builder /app/target/release/leafc /usr/local/bin/leafc

# Run the binary
CMD ["leafc"]
