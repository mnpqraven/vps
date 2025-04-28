FROM debian:bullseye AS builder
WORKDIR /usr/src/vps-rpc
RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y protobuf-compiler libprotobuf-dev curl build-essential pkg-config libssl-dev sqlite3
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
COPY . .

RUN pwd && ls

RUN . "$HOME/.cargo/env" && PROTOC=/usr/bin/protoc cargo install --path ./vps-rpc --bin vps-rpc

FROM rust:bullseye
COPY --from=builder /root/.cargo/bin/vps-rpc /usr/local/bin/vps-rpc

CMD ["vps-rpc"]
EXPOSE 5005
