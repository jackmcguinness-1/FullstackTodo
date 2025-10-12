# ===== Stage 1: Build =====
FROM rust:1.77 as builder
WORKDIR /app

# Copy everything and build release
COPY . .
RUN cargo build --release

# ===== Stage 2: Runtime =====
FROM debian:bookworm-slim
# Optional: install SSL libs if needed
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the Rust binary
COPY --from=builder /app/target/release/todoapi /usr/local/bin/todoapi

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/todoapi"]