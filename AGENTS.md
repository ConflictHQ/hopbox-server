# AGENTS.md — hopbox-server

Read `bootstrap.md` first.

- Git identity: `ragelink` / `Leo Mata <lmata@weareconflict.com>`. Set per-repo, never global.
- Rust binary — single static output, no web UI.
- PTY tests must run serially: `cargo test -- --test-threads=1`.
- License is non-commercial (SSPL or BSL TBD). Do NOT relicense without explicit approval.
