EXAMPLES := ${shell ls examples}

.PHONY: build test examples
build:
	@echo "Building locally..."
	@echo "------------------------------------------------------------------------"
	cargo build --all-features

examples:
	@echo "Building all examples..."
	@echo "------------------------------------------------------------------------"
	@for example in $(basename ${EXAMPLES}); do \
		set -e; \
		cargo build --example $$example; \
	done

test:
	@echo "Testing..."
	@echo "------------------------------------------------------------------------"
	cargo test --all-features

lint:
	@echo "Running clippy..."
	@echo "------------------------------------------------------------------------"
	cargo clippy

clean:
	cargo clean
