#!/usr/bin/env bash
set -euo pipefail

# Run from the directory as the repository root.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

# Run all of the tests and formatting operations.
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo hack check --feature-powerset
cargo hack build --feature-powerset
cargo +nightly rustdoc --all-features -- --cfg docsrs
cargo hack test --feature-powerset --lib
cargo test --doc --all-features
cargo test --all-features --test '*'
# We exclude integration tests because they take forever to run under Miri.
cargo +nightly miri test --all-features --lib
cargo +nightly miri test --all-features --doc
cargo +nightly docs-rs
