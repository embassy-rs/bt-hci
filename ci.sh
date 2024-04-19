#!/usr/bin/env bash

set -euxo pipefail

export RUSTFLAGS=-Dwarnings

cargo +nightly fmt -- --check

cargo clippy
cargo clippy --features embassy-time

cargo clippy --features defmt
cargo clippy --features defmt,embassy-time

cargo clippy --features log
cargo clippy --features log,embassy-time

cargo test --features embassy-time
