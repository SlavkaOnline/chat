FROM rust:1.65 AS builder

WORKDIR /app

RUN USER=root cargo new --bin chat

WORKDIR /app/chat

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo new --lib settings
COPY ./settings/Cargo.toml ./settings/Cargo.toml

RUN cargo new --lib migration 
COPY ./migration/Cargo.toml ./migration/Cargo.toml

RUN cargo build --release
RUN rm -rf ./src

RUN rm -rf ./settings
RUN rm -rf ./migration

COPY ./settings ./settings
COPY ./migration ./migration

COPY ./src ./src

RUN cargo build --release

FROM builder as test
WORKDIR /app/chat
ENTRYPOINT ["cargo", "test"]

FROM debian:buster-slim
COPY --from=builder /app/chat/target/release/chat ./app/chat

EXPOSE 80
ENTRYPOINT ["./app/chat"]