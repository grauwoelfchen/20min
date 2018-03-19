format:
	cargo fmt --all -- --write-mode=diff
.PHONY: format

fmt: | format
.PHONY: fmt

test:
	cargo test
.PHONY: test

coverage:
	./tools/build-kcov
	cargo build --tests
	#./tools/check-kcov integration_test
	./tools/check-kcov 20min
.PHONY: coverage

cov: | coverage
.PHONY: cov

build:
	cargo build
.PHONY: build

clean:
	cargo clean
.PHONY: clean

.DEFAULT_GOAL = coverage
default: coverage
