# Daily - Context Archive System for Claude Code
# Makefile for local development

.PHONY: build release install install-plugin reinstall clean test check fmt help web web-install

# Default target
.DEFAULT_GOAL := help

# Install web dependencies
web-install:
	cd web && npm install

# Build web frontend
web:
	cd web && npm run build

# Build debug version
build: web
	cargo build

# Build release version
release: web
	cargo build --release

# Install binary to ~/.cargo/bin
install: release
	cargo install --path .

# Install plugin (skills, commands, hooks) to user scope
install-plugin:
	daily install --scope user

# Full reinstall: build, install binary, reinstall plugin
reinstall: install install-plugin
	@echo ""
	@echo "Reinstall complete!"
	@echo "  - Binary installed to ~/.cargo/bin/daily"
	@echo "  - Plugin installed to ~/.claude/"

# Run tests
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Format check
fmt-check:
	cargo fmt --check

# Lint with clippy
lint:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Show help
help:
	@echo "Daily - Development Makefile"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  web-install    Install web frontend dependencies"
	@echo "  web            Build web frontend only"
	@echo "  build          Build debug version (includes web)"
	@echo "  release        Build release version (includes web)"
	@echo "  install        Build release and install binary"
	@echo "  install-plugin Install skills and hooks to ~/.claude/"
	@echo "  reinstall      Full reinstall (build + binary + plugin)"
	@echo "  test           Run tests"
	@echo "  test-verbose   Run tests with output"
	@echo "  check          Check code without building"
	@echo "  fmt            Format code"
	@echo "  fmt-check      Check code formatting"
	@echo "  lint           Run clippy linter"
	@echo "  clean          Clean build artifacts"
	@echo "  help           Show this help"
