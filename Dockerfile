FROM rust:1-buster AS builder
WORKDIR /app
RUN cargo install cargo-build-dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build-dependencies --release
COPY src ./src

RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
#RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=builder /app/target/release/main-api /app/main-api
ENTRYPOINT [ "/app/main-api" ]
