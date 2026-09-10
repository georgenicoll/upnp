#!/usr/bin/env bash
set -euo pipefail

ensure_tool() {
  local subcommand="$1"
  local crate_name="$2"

  if ! cargo "$subcommand" --version >/dev/null 2>&1; then
    echo "==> installing ${crate_name} (missing cargo ${subcommand})"
    cargo install "$crate_name" --locked
  fi
}

run_step() {
  local name="$1"
  shift
  echo "==> ${name}"
  cargo "$@"
}

ensure_tool audit cargo-audit
ensure_tool deny cargo-deny

run_step fmt fmt --all -- --check
run_step clippy clippy --all-targets --all-features -- -D warnings
run_step test test --all-targets --all-features
run_step audit audit
run_step deny deny check

echo "all checks passed"
