# Use Debian Bullseye in both stages to ensure GLIBC compatibility
FROM docker.io/library/debian:bullseye-slim AS builder

# Install Rust dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential && \
        curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set environment variables
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo files and source code
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Use the same base image for the runtime
FROM docker.io/library/debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/fortune_backend /usr/local/bin/fortune_backend

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["fortune_backend"]

