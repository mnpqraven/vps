# VPS control tools

## Note on ports

The default following ports are used:

- 4000: API server
- 4001: RPC server

## manual deployments

### build

```
cargo install --path <package-name>
sudo systemctl restart <package-name.service>
```
