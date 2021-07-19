#!/usr/bin/env bash

echo "📝 Format"
cargo +nightly fmt --all

echo "👮 Lint"
cargo +nightly clippy --all