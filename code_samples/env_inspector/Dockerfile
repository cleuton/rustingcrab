# Stage 1: compile the `inspect` binary
FROM rust:1-slim AS builder
WORKDIR /usr/src/env_inspector

# Copy your entire crate (lib + bin directory)
COPY . .

# Build only the `inspect` binary in release mode
RUN cargo build --release --bin inspect

# Stage 2: minimal runtime image
FROM debian:bookworm-slim

# Copy just the compiled binary
COPY --from=builder /usr/src/env_inspector/target/release/inspect /usr/local/bin/inspect

# Default command
ENTRYPOINT ["inspect"]
