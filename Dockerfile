# Stage 1: Builder
FROM rust:1.83 AS builder

RUN rustup target add x86_64-unknown-linux-gnu

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy your source code
COPY . .

# Build with static linking
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app
EXPOSE 8888

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/harpchat /app/harpchat

# Set the binary as the entrypoint
CMD ["/app/harpchat"]
