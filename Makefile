# verify -- {{{
verify\:check:  ## Check rust syntax
	@cargo check --all -v
.PHONY: verify\:check

verify\:format:  ## Check format without changes (alias: verify:fmt, fmt)
	@cargo fmt --all -- --check
.PHONY: format

verify\:fmt: | verify\:format
.PHONY: verify\:fmt

format: | verify\:format
.PHONY: format

fmt: | verify\:format
.PHONY: fmt

verify\:lint:  ## Check code style using clippy (alias: lint)
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: | verify\:lint
.PHONY: lint

verify\:all: | verify\:check verify\:format verify\:lint  ## Check by all verify:xxx targets
.PHONY: verify\:all

verify: | verify\:check  ## Same as verify:check
.PHONY: verify
# }}}

# test -- {{{
test\:unit:  ## Run only unit tests
	@cargo test --bin 20min
.PHONY: test\:unit

test\:integration:  ## Run integration tests only
	@cargo test --test integration
.PHONY: test\:integration

test\:all:  ## Run all test targets
	@cargo test --tests
.PHONY: test\:all

test: | test\:unit  ## Same as test:unit
.PHONY: test
# }}}

# coverage -- {{{
coverage\:unit:  ## Generate coverage report of unit tests using kcov (alias: cov:unit)
	@cargo test --bin 20min --no-run
	@./.tool/check-kcov 20min
.PHONY: test\:coverage

cov\:unit: | coverage\:unit
.PHONY: cov\:unit

coverage\:integration:  ## Generate coverage report of integration tests (alias cov:integration)
	@cargo test  --test integration --no-run
	@./.tool/check-kcov integration
.PHONY: coverage\:integration

cov\:integration: coverage\:integration
.PHONY: cov\:integration

coverage: | coverage\:unit  ## Same as coverage:unit (alias: cov)
.PHONY: coverage

cov: | coverage
.PHONY: cov
# }}}

# documentation -- {{{
document:  ## Generate documentation files (alias: doc)
	cargo rustdoc -- \
		--document-private-items -Z unstable-options --display-warnings
.PHONY: document

doc: | document
.PHONY: doc
# }}}

# build -- {{{
build\:debug:  ## Create debug build
	cargo build
.PHONY: build\:debug

build\:release:  ## Create release build
	cargo build --release
.PHONY: build\:release

build: | build\:debug  ## Same as build:debug
.PHONY: build
# }}}

# other utilities -- {{{
clean:  ## Clean up
	@cargo clean
.PHONY: clean

help:  ## Display this message
	@grep -E '^[0-9a-z\:\\]+: ' $(MAKEFILE_LIST) | grep -E '  ## ' | \
		sed -e 's/\(\s|\(\s[0-9a-z\:\\]*\)*\)  /  /' | tr -d \\\\ | \
		awk 'BEGIN {FS = ":  ## "};" \
		  "{printf "\033[38;05;222m%-21s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = test
default: test
# }}}
