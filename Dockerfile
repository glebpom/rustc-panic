FROM rust:1.25.0-stretch

ADD . /code
WORKDIR /code

RUN cargo check --lib

