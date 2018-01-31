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
	./kcov/bin/kcov --include-path src target/coverage target/debug/20min-*
	grep -oE 'covered":"([0-9]*\.[0-9]*|[0-9]*)"' target/coverage/index.json | \
		grep -oE '[0-9]*\.[0-9]*|[0-9]*'
.PHONY: coverage

cov: | coverage
.PHONY: cov

.DEFAULT_GOAL = coverage
default: coverage
