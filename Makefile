# -- vet

vet\:check:  ## Check rust syntax
	cargo check --all -v
.PHONY: vet\:check

vet\:format:  ## Check format without changes (alias: vet:fmt, fmt)
	cargo fmt --all -- --check
.PHONY: format

vet\:fmt: | vet\:format
.PHONY: vet\:fmt

fmt: | vet\:format
.PHONY: fmt

vet\:lint:  ## Check code style using clippy (alias: lint)
	cargo clippy --all-targets
.PHONY: vet\:lint

lint: | vet\:lint
.PHONY: lint

vet\:all: | vet\:check vet\:format vet\:lint  ## Check by all vet:xxx targets
.PHONY: vet\:all

vet: | vet\:check  ## Same as vet:check
.PHONY: vet


# -- test

test\:bin:  ## Run only tests for bin (20min)
	cargo test --bin 20min
.PHONY: test\:bin

test\:integration:  ## Run integrations test only
	cargo test --test integration_test
.PHONY: test\:integration

test\:all:  ## Run all test targets
	cargo test --tests
.PHONY: test\:all

test: | test\:bin   ## Same as test:bin
.PHONY: test


# -- coverage

coverage\:bin:  ## Generate coverage report of unit tests using kcov (alias: cov:bin)
	cargo test --bin 20min --no-run
	./.tools/check-kcov 20min
.PHONY: test\:coverage

cov\:bin: | coverage\:bin
.PHONY: cov\:bin

coverage\:integration:  ## Generate coverage report of integration tests (alias cov:integration)
	cargo test  --test integration_test --no-run
	./.tools/check-kcov integration_test
.PHONY: coverage\:integration

cov\:integration: coverage\:integration
.PHONY: cov\:integration

coverage: | coverage\:bin  ## Same as coverage:bin (alias: cov)
.PHONY: coverage

cov: | coverage
.PHONY: cov

# -- doc

document:  ## Generate documentation files (alias: doc)
	cargo rustdoc -- --document-private-items -Z unstable-options --display-warnings
.PHONY: document

doc: | document
.PHONY: doc


# -- build

build\:debug:  ## Create debug build
	cargo build
.PHONY: build\:debug

build\:release:  ## Create release build
	cargo build --release
.PHONY: build\:release

build: | build\:debug  ## Same as build:debug
.PHONY: build


# -- other utilities

clean:  ## Clean up
	cargo clean
.PHONY: clean

help:  ## Display this message
	@grep -E '^[0-9a-z\:\\]+: ' $(MAKEFILE_LIST) | grep -E '  ## ' | \
	  sed -e 's/\(\s|\(\s[0-9a-z\:\\]*\)*\)  /  /' | tr -d \\\\ | \
	  awk 'BEGIN {FS = ":  ## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' | \
	  sort
.PHONY: help

.DEFAULT_GOAL = test
default: test
