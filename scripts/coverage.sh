#!/usr/bin/env bash
set -euo pipefail

# Run from the directory as the repository root.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

# Process the coverage request.
cargo tarpaulin \
    --all-features --doc --tests \
    --out html \
    --output-dir target \
    --include-files 'typed-ident/src/*'
