FROM rust:1.61 AS builder

WORKDIR /app

RUN USER=root cargo new --bin chat

WORKDIR /app/chat
COPY . .

RUN cargo build --release

FROM builder as test
WORKDIR /app/chat/src
ENTRYPOINT ["cargo", "test", "--release"]

FROM debian:buster-slim
COPY --from=builder /app/chat/target/release/chat ./app/chat

EXPOSE 80
ENTRYPOINT ["./app/chat"]