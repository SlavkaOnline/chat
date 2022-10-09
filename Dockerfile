FROM rust:1.61 AS builder

WORKDIR /app

RUN USER=root cargo new --bin chat

WORKDIR /app/chat

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/chat*

RUN cargo build --release

FROM builder as test
WORKDIR /app/chat/src
ENTRYPOINT ["cargo", "test", "--release"]

FROM debian:buster-slim
COPY --from=builder /app/chat/target/release/chat ./app/chat

EXPOSE 80
ENTRYPOINT ["./app/chat"]