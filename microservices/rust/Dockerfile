FROM rust:1.61

RUN apt-get update
RUN cargo fix --edition

WORKDIR /app

ENTRYPOINT cargo run
