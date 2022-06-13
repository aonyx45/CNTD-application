FROM rust:1.49

RUN apt-get update

RUN apt-get upgrade

RUN apt-get install git wget curl rustc cargo

WORKDIR /app

ENTRYPOINT cargo run