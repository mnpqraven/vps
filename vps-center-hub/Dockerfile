FROM debian:bullseye as builder
WORKDIR /usr/src/nas-ws
RUN apt-get update && apt-get upgrade && apt-get install -y protobuf-compiler libprotobuf-dev curl build-essential pkg-config libssl-dev sqlite3
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
COPY . .

RUN pwd && ls

RUN . "$HOME/.cargo/env" && PROTOC=/usr/bin/protoc cargo install --path ./crates/vps-center-hub

FROM rust:bullseye
COPY --from=builder /root/.cargo/bin/vps-center-hub /usr/local/bin/vps-center-hub

CMD ["vps-center-hub"]
EXPOSE 5005

