FROM rust:1.49

RUN apt-get update

RUN apt-get install git wget curl rustc cargo -y

WORKDIR /app

ENTRYPOINT cargo run
