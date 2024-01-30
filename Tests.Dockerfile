FROM ubuntu:latest

RUN apt update
RUN apt install curl -y
RUN apt install build-essential -y
RUN apt install protobuf-compiler -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN /root/.cargo/bin/rustup update 1.75.0

RUN apt install python3.10 -y

COPY ./Ficus/ ./pmide/ficus/
COPY ./bxes/ ./pmide/bxes/

ENTRYPOINT /root/.cargo/bin/cargo test --manifest-path ./pmide/ficus/src/rust/ficus_backend/Cargo.toml --release
