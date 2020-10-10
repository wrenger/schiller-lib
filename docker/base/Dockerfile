FROM rust

RUN apt-get update && \
    apt-get install --assume-yes llvm-dev libclang-dev clang gcc-mingw-w64 && \
    rustup target add x86_64-pc-windows-gnu
