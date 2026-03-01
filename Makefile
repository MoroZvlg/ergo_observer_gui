.PHONY: help run build fmt lint check test clean build-wasm run-wasm run-dev

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*##"; printf "\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  %-15s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

fmt: ## Format code with rustfmt
	cargo fmt

lint: ## Run clippy linter
	cargo clippy -- -W clippy::all
	
test: ## Run tests
	cargo test
	
check: fmt lint test ## Run formatter, linter and tests
	@echo "✓ Format, lint and tests passed"

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

run: ## Run the app natively
	cargo run

build: ## Build the app (release mode)
		cargo build --release
		
build-wasm: ## Build for WebAssembly using Trunk
	trunk build --release

run-wasm: ## Start WASM dev server with hot reload (http://localhost:8080)
	trunk serve

run-dev: ## Run in dev mode with optimizations
	cargo run --profile dev
