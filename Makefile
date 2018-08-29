check:  ## Check rust syntax
	cargo check --all -v
.PHONY: check

format:  ## Check format without changes (alias: fmt)
	cargo fmt --all -- --check
.PHONY: format

fmt: | format
.PHONY: fmt

lint:  ## Check code style using clippy
	cargo clippy --all-targets
.PHONY: lint

test:  ## Run unit tests and integration tests
	cargo test
.PHONY: test

coverage:  ## Generate coverage report of unit tests using kcov (alias: cov)
	# cargo build --tests
	cargo test --bin 20min --no-run
	#./.tools/check-kcov integration_test
	./.tools/check-kcov 20min kcov
.PHONY: coverage

cov: | coverage
.PHONY: cov

document:  ## Generate documentation files (alias: doc)
	cargo rustdoc -- --document-private-items -Z --display-warnings
.PHONY: document

doc: | document
.PHONY: doc

build:  ## Run debug build
	cargo build
.PHONY: build

release:  ## Create release build
	cargo build --release
.PHONY: release

clean:  ## Spruce up
	cargo clean
.PHONY: clean

help:  ## Display this message
	@grep -E '^[a-z]+:  ## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":  ## "}; {printf "\033[36m%-10s\033[0m %s\n", $$1, $$2}'
.PHONY: help

.DEFAULT_GOAL = coverage
default: coverage
