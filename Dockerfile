################
##### Builder
################

FROM rust:latest as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new nsfw-check-server 

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/nsfw-check-server/

# Set the working directory
WORKDIR /usr/src/nsfw-check-server

# This is a dummy build to get the dependencies cached.
RUN cargo build --release

# Now copy in the rest of the sources
COPY src /usr/src/nsfw-check-server/src/

# Copy model
COPY model.onnx /usr/src/nsfw-check-server/model.onnx

## Touch main.rs to prevent cached release build
RUN touch /usr/src/nsfw-check-server/src/main.rs

# This is the actual application build.
RUN cargo build --release




################
##### Runtime
################

FROM ubuntu:latest AS runtime 

# Install nano & certificates
RUN apt update && apt install -y nano ca-certificates && update-ca-certificates

# Copy application binary from builder image
COPY --from=builder /usr/src/nsfw-check-server/target/release/nsfw-check-server /usr/local/bin/

EXPOSE 6969 

# Set workdir to folder with binary and config link
WORKDIR /usr/local/bin/

# Run the application
CMD ["/usr/local/bin/nsfw-check-server"]
