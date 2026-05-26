# hopbox-server bootstrap

## What this is

hopbox-server is a self-hosted Rust session daemon. It is a single static binary, Docker-friendly, with no web UI. Configuration is done via TOML + environment variables.

## What it does

- Accepts client connections from hopbox-desktop (and other clients) over a configurable transport.
- Allocates a PTY (pseudo-terminal) per session.
- Keeps the PTY alive when clients disconnect — sessions are server-resident, not client-resident.
- Allows clients to reconnect and reattach to existing sessions.
- Persists AI context server-side so it survives client disconnects and reconnects.

## Module structure

```
src/
  main.rs           — entry, listener setup, config load
  server.rs         — connection handler (accepts WS or TLS connections)
  session_store.rs  — in-memory store of active sessions (future: disk persistence)
  session.rs        — one session: PTY + AI context + connected clients list
  config.rs         — TOML config loader
```

## Configuration

Config file is loaded from `/etc/hopbox/server.toml` by default. Override the location with the `HOPBOX_CONFIG` environment variable.

Config schema:

```toml
[transport]
mode = "websocket"   # "websocket" | "tls"
port = 7878
tls_cert = "/etc/hopbox/cert.pem"
tls_key  = "/etc/hopbox/key.pem"

[auth]
token = "secret-token-here"   # clients must present this

[sessions]
max_sessions = 100
idle_timeout_secs = 3600
```

## Transport negotiation

The client URL scheme determines the transport:

- `ws://host:port` or `wss://host:port` → WebSocket
- `hbox://host:port` → raw TLS

## Key dependencies

- `hopbox-core` (git dep) — shared protocol types and PTY abstractions
- `tokio` — async runtime
- `tokio-tungstenite` — WebSocket transport
- `rustls` — TLS transport
- `axum` — HTTP only for the health-check endpoint
- `serde` + `toml` — config parsing
- `tracing` + `tracing-subscriber` — structured logging
- `anyhow` — error plumbing
- `uuid` — session IDs

## Build

```
cargo build --release
```

The output is a single static binary at `target/release/hopbox-server`.

## Docker

```
docker build -t hopbox-server .
```

Note: the Dockerfile is a future task and is not yet committed.

## Test

```
cargo test -- --test-threads=1
```

PTY tests must run serially — running them in parallel causes file-descriptor and controlling-terminal contention on macOS and Linux.

## License

Non-commercial — SSPL or BSL is to be determined. Do not relicense without explicit approval.
