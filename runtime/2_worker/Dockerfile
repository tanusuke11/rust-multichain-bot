# Use the official Rust image as a base
FROM rust:1.70

# Set the working directory
WORKDIR /app

# Remove the dummy source files
RUN rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Set the entry point
CMD ["./target/release/worker"]
