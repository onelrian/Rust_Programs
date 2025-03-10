# Use the official Rust image as the base
FROM rust:latest as builder

# Install the musl target
RUN rustup target add x86_64-unknown-linux-musl

# Set the working directory
WORKDIR /app

# Copy the project files
COPY . .

# Build the project in release mode for the musl target
RUN cargo build --release --target x86_64-unknown-linux-musl

# Use a minimal Alpine Linux image for the final container
FROM alpine:latest

# Copy the binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/shape_area_calculator /usr/local/bin/shape_area_calculator

# Set the entry point
ENTRYPOINT ["shape_area_calculator"]