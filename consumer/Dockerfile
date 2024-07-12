FROM rust:1.74.0

WORKDIR /usr/src/app

# Install any necessary dependencies
# Replace 'your-dependencies' with actual package names your app needs
RUN apt-get update && \
    apt-get install -y your-dependencies && \
    rm -rf /var/lib/apt/lists/*

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Set the startup command
CMD ["./target/release/consumer"]