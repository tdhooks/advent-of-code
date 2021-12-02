FROM rust:alpine

ARG TARGET_PROJECT=hello_rust

WORKDIR /golf

COPY ./${TARGET_PROJECT} /golf

RUN rustc main.rs

ENTRYPOINT ["./main", "./input/input.txt"]
