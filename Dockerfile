################
##### Builder
################

FROM rust:latest AS builder

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

# Remove debug information from build
RUN strip /usr/src/nsfw-check-server/target/release/nsfw-check-server


################
##### Runtime
################

FROM cgr.dev/chainguard/wolfi-base:latest AS runtime 

# Install openssl library, which provides required libssl.so.3 file.
# The --no-cache flag to keep the final image size small.
RUN apk update && apk add --no-cache openssl

# Copy application binary from builder image
COPY --from=builder /usr/src/nsfw-check-server/target/release/nsfw-check-server /usr/local/bin/

EXPOSE 6969 

# Set workdir to non-root location
WORKDIR /app

# Run the application
CMD ["/usr/local/bin/nsfw-check-server"]
