FROM rust

RUN apt-get update && \
    apt-get install --assume-yes llvm-dev libclang-dev clang && \
    cargo install cross
