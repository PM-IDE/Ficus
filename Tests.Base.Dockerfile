FROM ubuntu:latest

RUN apt update
RUN apt install curl -y
RUN apt install build-essential -y
RUN apt install protobuf-compiler -y
RUN apt-get -y install python3-pip

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV cargo="/root/.cargo/bin/cargo"
ENV rustup="/root/.cargo/bin/rustup"
ENV python="/bin/python3.10"

RUN $rustup update 1.75.0

RUN apt install python3.10 -y

COPY ./Ficus/ ./pmide/ficus/
COPY ./bxes/ ./pmide/bxes/

RUN $cargo build --manifest-path /pmide/ficus/src/rust/ficus_backend/Cargo.toml --release