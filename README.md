# monkeynuthead-upnp

Minimal Rust starter repository for learning UPnP step-by-step.

## Quick Start

1. Verify toolchain:

```powershell
rustup show
cargo --version
```

2. Build and run:

```powershell
cargo build
cargo run
```

3. Configure log verbosity (optional):

```powershell
$env:RUST_LOG = "info,monkeynuthead_upnp=debug"
cargo run
```

## Milestone 2 Discovery Commands

Run one-shot responder (default bind is `0.0.0.0:1900`):

```powershell
cargo run --bin ssdp-server
```

Run discovery client (default target is multicast `239.255.255.250:1900`):

```powershell
cargo run --bin ssdp-client
```

Local loopback simulation (useful when multicast is restricted):

Terminal 1:

```powershell
$env:SSDP_BIND = "127.0.0.1:1901"
cargo run --bin ssdp-server
```

Terminal 2:

```powershell
$env:SSDP_TARGET = "127.0.0.1:1901"
$env:SSDP_BIND = "127.0.0.1:0"
$env:SSDP_ST = "upnp:rootdevice"
cargo run --bin ssdp-client
```

## Milestone 3 Device Description

Run the HTTP description endpoint:

```powershell
cargo run --bin device-description-server
```

The default SSDP `LOCATION` header already points at `http://127.0.0.1:8000/device.xml`, so the discovery responder and the HTTP endpoint fit together out of the box.

If you want to change the local demo target, set `HTTP_BIND` and `SSDP_LOCATION` together before running the binaries.

## Quality and Security Tooling

This repository includes:

- `rustfmt` for formatting (`rust-toolchain.toml`)
- `clippy` for linting (`rust-toolchain.toml`)
- `cargo-audit` for RustSec advisory scanning (CI)
- `cargo-deny` for dependency, source, and license checks (`deny.toml`)

Install local scanner tools once:

```powershell
cargo install cargo-audit --locked
cargo install cargo-deny --locked
```

Run checks locally:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo audit
cargo deny check
```

Helper aliases are defined in `.cargo/config.toml`:

```powershell
cargo fmt-check
cargo lint
cargo qa
cargo run-checks
```

`cargo run-checks` uses a small Rust helper binary in `src/bin/run-checks.rs`.

By default it runs required checks (`fmt`, `clippy`, `check`, `test`) and skips `audit`/`deny` if those subcommands are not installed.

Strict mode (fail if `audit` or `deny` is missing):

```powershell
$env:RUN_CHECKS_STRICT = "1"
cargo run-checks
```

Script entrypoints (same checks):

```powershell
./scripts/run-checks.ps1
```

```bash
./scripts/run-checks.sh
```
