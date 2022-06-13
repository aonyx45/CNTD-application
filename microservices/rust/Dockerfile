FROM rust:1.61

RUN apt-get update
RUN rustup default nightly
RUN rustup update

WORKDIR /app

ENTRYPOINT cargo run
