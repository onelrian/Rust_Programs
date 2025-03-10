# Use the official Rust image to build the application
FROM rust:latest as builder

# Set the working directory (matches the mount point in your VM)
WORKDIR /home/ubuntu/longest-substring

# Copy the Cargo.toml and Cargo.lock to the container (for caching dependencies)
COPY Cargo.toml Cargo.lock ./

# Copy the source code into the container
COPY src ./src

# Build the Rust application in release mode
RUN cargo build --release

# Check the contents of the release folder to ensure the binary exists
RUN ls -alh /home/ubuntu/longest-substring/target/release

# Create a new minimal image to run the application
FROM debian:bookworm-slim

# Install necessary dependencies for runtime
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder image
COPY --from=builder /home/ubuntu/longest-substring/target/release/longest-substring /usr/local/bin/longest_substring

# Set the entry point to run the program
ENTRYPOINT ["/usr/local/bin/longest_substring"]