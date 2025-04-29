FROM debian:bullseye AS builder

WORKDIR /usr/src/vps-api
RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y protobuf-compiler libprotobuf-dev curl build-essential pkg-config libssl-dev sqlite3
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
COPY . .
RUN pwd && ls
RUN . "$HOME/.cargo/env" && PROTOC=/usr/bin/protoc cargo install --path ./vps-api --bin vps-api

FROM rust:bullseye AS runner

COPY --from=builder /root/.cargo/bin/vps-api /usr/local/bin/vps-api
CMD ["vps-api"]
EXPOSE 5000
