.PHONY: all build test clean example-generate example-verify

all: build test

build:
	cargo build

test:
	cargo test

clean:
	cargo clean
	rm -rf test_files

example-generate:
	cargo run --bin generate_example

example-verify:
	cargo run --bin verify_example

# Run all examples
examples: example-generate example-verify

# Help command
help:
	@echo "Available commands:"
	@echo "  make build          - Build the project"
	@echo "  make test          - Run tests"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make example-generate - Generate example proof"
	@echo "  make example-verify   - Verify example proof"
	@echo "  make examples       - Run all examples"
	@echo "  make help          - Show this help message" 