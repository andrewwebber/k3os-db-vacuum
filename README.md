# Repair K3OS large db wal usage

## Build binary

```bash
cargo build --release --target=x86_64-unknown-linux-musl
```

## Deploy binary to node

```bash
scp ./target/x86_64-unknown-linux-musl/release/vacuum rancher@XXX:/home/rancher/
```

## Repair Node

```bash
chmod +x ./vacuum
sudo service k3s-service stop
sudo ./vacuum /var/lib/rancher/k3s/server/db/state.db
sudo service k3s-service start
```
