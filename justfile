layout:
    zellij -l .zellij/servers.kdl

dev-admin:
    cargo leptop watch
dev-rpc:
    cargo run --package vps-rpc
dev-api:
    cargo run --package vps-api

ui rpc:
    grpcui -port 5006 -plaintext localhost:5005 || echo "is the gRPC server running on port 5005?"

migrate name:
    cargo sqlx migrate add -r {{name}}

test-db:
    bacon test-db

test:
    bacon test
