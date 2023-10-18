# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Build the Rust application
RUN cargo build

# Specify the command to run when the container starts (e.g., running your project)
CMD ["cargo", "run"]
