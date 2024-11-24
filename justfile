set dotenv-load

# List all just receipes
default:
    @just --list

# Install require tools
requirements:
    @echo "Install Rust nightly for formatting"
    rustup toolchain add nightly
    @echo "Install cargo-nextest to run test"
    cargo install cargo-nextest
    @echo "Install "bacon for tdd"
    cargo install bacon
    @echo "Install cargo-audit for audit"
    cargo install cargo-audit
    @echo "Install cargo-deny for audit"
    cargo install cargo-deny

# Run TDD mode
tdd:
    bacon nextest

# Launch tests
test:
    @echo "🧪 Testing..."
    cargo nextest run
    cargo test --doc

# Format the code
format:
    cargo +nightly fmt

# Format the code
lint:
    @echo "🎩 Linting..."
    cargo check --all-features
    cargo clippy --all-features

# Check the code (formatting, lint, and tests)
check:
    @echo "🦀 Check formatting..."
    cargo +nightly fmt --all -- --check
    @just lint
    @just test

# Audit (security issue, licences)
audit:
    @echo "🚨 Audit CVE..."
    cargo audit

    @echo "🪪 Check licences..."
    cargo deny check

# Build the documentation
doc:
    cargo doc

# Build in production mode
build:
    @just check
    echo "⚙️ Build"
    cargo build --release
