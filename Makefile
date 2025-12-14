# Makefile
# Project: onda

PROJECT_NAME := onda

.PHONY: all build test run fmt clippy clean check-deps install-tools

# Default target: format, lint, test, and build
all: fmt clippy test build

# Install necessary development tools
install-tools:
	@echo "Installing Rust tools..."
	rustup component add rustfmt clippy
	cargo install cargo-udeps --locked
	pip install pre-commit # Requires Python

# Format code using standard Rust style
fmt:
	cargo fmt

# Linter (Clippy): Fails if warnings are found
# -D warnings ensures the build fails on any warning (strict mode)
clippy:
	cargo clippy -- -D warnings

# Run unit tests
test:
	cargo test --workspace

# Build release version
build:
	cargo build --release

# Run project
run:
	cargo run --release

# Check for unused dependencies in Cargo.toml
check-deps:
	cargo +nightly udeps || echo "Nightly toolchain required for udeps. Skipping..."

# Clean build artifacts
clean:
	cargo clean

# Install git hooks
setup-hooks:
	pre-commit install
