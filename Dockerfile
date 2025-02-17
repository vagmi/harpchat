# Stage 1: Builder
FROM rust:1.83 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy your source code
COPY . .

# Build with static linking
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookwork-slim

WORKDIR /app
EXPOSE 8888

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/harpchat /app/harpchat

# Set the binary as the entrypoint
ENTRYPOINT ["/app/harpchat"]
