build:
	cargo build
.PHONY: build

clean:
	cargo clean
.PHONY: clean

test:
	cargo test
.PHONY: test

coverage:
	./bin/build-kcov
	cargo test
	./bin/check-kcov 20min
.PHONY: coverage

cov: | coverage
.PHONY: cov

.DEFAULT_GOAL = coverage
default: coverage
