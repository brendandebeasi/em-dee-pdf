# Multi-stage build for md-pdf

# Stage 1: Build the Rust binary
FROM rust:1.84-slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime with Typst
FROM ghcr.io/typst/typst:latest AS typst

FROM debian:bookworm-slim

# Install minimal dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    fontconfig \
    fonts-dejavu-core \
    && rm -rf /var/lib/apt/lists/*

# Copy md-pdf binary
COPY --from=builder /app/target/release/md-pdf /usr/local/bin/

# Copy typst binary
COPY --from=typst /bin/typst /usr/local/bin/

# Create working directory
WORKDIR /work

# Default entrypoint
ENTRYPOINT ["md-pdf"]
