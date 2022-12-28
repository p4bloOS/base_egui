#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.
set -eux

cargo check --workspace --all-targets
cargo check --workspace --all-features --lib --target wasm32-unknown-unknown
# cargo fmt --all -- --check #Esta línea comprobaría que todo coincide con la plantilla original de eframe
cargo clippy --workspace --all-targets --all-features --  -D warnings -W clippy::all
cargo test --workspace --all-targets --all-features
cargo test --workspace --doc
trunk build
