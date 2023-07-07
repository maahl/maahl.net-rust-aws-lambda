# Use a multi-stage build to minimize the size of the final image
FROM rust:1.73-bookworm AS builder

WORKDIR /build

# Installing cargo lambda with pip also installs zig
RUN apt update && \
    apt install -y python3-pip && \
    pip3 install cargo-lambda --break-system-packages

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo lambda build --release --arm64

# The final image will only contain the resulting binary
FROM public.ecr.aws/lambda/provided:al2-arm64

COPY --from=builder /build/target/lambda/rust-aws-lambda/bootstrap /bootstrap

ENTRYPOINT ["/bootstrap"]
