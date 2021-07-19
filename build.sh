#!/usr/bin/env bash

echo "🧪 Tests"
cargo test

echo "⚙️ Build"
cargo build
cargo build --release
