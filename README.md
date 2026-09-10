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
```
