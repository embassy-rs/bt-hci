#!/usr/bin/env bash

set -euxo pipefail

export RUSTFLAGS=-Dwarnings

cargo +nightly fmt -- --check

cargo clippy

cargo clippy -p bt-hci --features embassy-time
cargo clippy -p bt-hci --features defmt
cargo clippy -p bt-hci --features defmt,embassy-time
cargo clippy -p bt-hci --features log
cargo clippy -p bt-hci --features log,embassy-time
cargo clippy -p bt-hci --features serde

cargo clippy -p bt-hci-transport --features defmt
cargo clippy -p bt-hci-serial --features defmt
cargo clippy -p bt-hci-linux
cargo clippy -p bt-hci-usb 

cargo test -p bt-hci --features embassy-time,serde
