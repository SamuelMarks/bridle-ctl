.PHONY: help install_base install_deps docs_rust docs_angular build test docs

help:
	@echo "Available targets:"
	@echo "  install_base   - Install language runtime and base dependencies (gcc, cmake, etc.)"
	@echo "  install_deps   - Install local dependencies (Cargo, npm, MkDocs)"
	@echo "  docs_rust      - Generate Rust documentation"
	@echo "  docs_angular   - Generate Angular documentation"
	@echo "  build          - Build the Rust and Angular projects"
	@echo "  test           - Run tests for Rust and Angular projects"
	@echo "  docs           - Build the unified Material for MkDocs site"

install_base:
	@echo "Installing base dependencies..."
	@if [ "$$(uname)" = "Darwin" ]; then \
		echo "Detected macOS. Using Homebrew..."; \
		brew install cmake pkg-config python3 node; \
	elif [ -n "$$(command -v apt-get)" ]; then \
		echo "Detected Debian/Ubuntu. Using apt-get..."; \
		sudo apt-get update && sudo apt-get install -y build-essential cmake pkg-config python3 python3-pip python3-venv nodejs npm; \
	else \
		echo "Unsupported OS or missing package manager. Please install gcc, cmake, pkg-config, python3, and nodejs manually."; \
	fi

install_deps:
	@echo "Installing project dependencies..."
	cargo fetch
	cd bridle-ui && npm install
	python3 -m pip install --upgrade pip mkdocs-material

docs_rust:
	@echo "Generating Rust docs..."
	cargo doc --no-deps --document-private-items

docs_angular:
	@echo "Generating Angular docs..."
	cd bridle-ui && npx compodoc -p tsconfig.doc.json

build:
	@echo "Building projects..."
	cargo build
	cd bridle-ui && npm run build

test:
	@echo "Running tests..."
	cargo test
	cd bridle-ui && npm test -- --watch=false

docs: docs_rust docs_angular
	@echo "Building MkDocs site..."
	python3 -m mkdocs build
	@echo "Merging Rust docs into MkDocs site..."
	mkdir -p site/rust
	cp -R target/doc/* site/rust/
	@echo "Merging Angular docs into MkDocs site..."
	mkdir -p site/angular
	cp -R bridle-ui/docs/* site/angular/
	@echo "✅ Docs successfully generated in ./site"

precommit:
	@echo "Running pre-commit hook..."
	./.git/hooks/pre-commit
