PACKAGE = twenty-minutes
MODULE = twenty_minutes
BINARY = 20min

# verify
verify\:check: ## Check rust syntax [alias: check]
	@cargo check --all -v
.PHONY: verify\:check

check: verify\:check
.PHONY: check

verify\:format: ## Check format without changes [alias: verify:fmt, fmt]
	@cargo fmt --all -- --check
.PHONY: format

verify\:fmt: verify\:format
.PHONY: verify\:fmt

format: verify\:format
.PHONY: format

fmt: verify\:format
.PHONY: fmt

verify\:lint: ## Check code style using clippy [alias: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: verify\:lint
.PHONY: lint

verify\:all: verify\:check verify\:format verify\:lint ## Run all verify targets
.PHONY: verify\:all

verify: verify\:check ## Same as verify:check
.PHONY: verify

# test
test\:bin: ## Run only unit tests for binary (20min)
	@cargo test --bin $(BINARY)
.PHONY: test\:bin

test\:lib: ## Run only unit tests for library (twenty_minutes)
	@cargo test --lib
.PHONY: test\:lib

test\:e2e: ## Run e2e tests
	@cargo test --test e2e
.PHONY: test\:e2e

test\:all: ## Run all test targets
	@cargo test --tests
.PHONY: test\:all

test: test\:lib ## Same as test:lib
.PHONY: test

# coverage
coverage\:bin: ## Generate a coverage report of tests for binary [alias: cov:bin]
	set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/bin"; \
	cargo test --bin $(BINARY) --no-run --target-dir=$${target_dir}; \
	result=($${target_dir}/index.js*); \
	if [ -f $${result}[0] ]; then \
		rm "$${target_dir}/index.js*"; \
	fi; \
	file=($$target_dir/debug/deps/$(BINARY)-*); \
	kcov --verify --include-path=$$dir/src $$target_dir $${file[0]}; \
	grep 'index.html' $$target_dir/index.js* | \
		grep --only-matching --extended-regexp \
		'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'
.PHONY: coverage\:bin

cov\:bin: coverage\:bin
.PHONY: cov\:bin

coverage\:lib: ## Generate a coverage report of tests for library [alias: cov:lib]
	set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/lib"; \
	cargo test --lib --no-run --target-dir=$${target_dir}; \
	result=($${target_dir}/index.js*); \
	if [ -f $${result}[0] ]; then \
		rm "$${target_dir}/index.js*"; \
	fi; \
	file=($$target_dir/debug/deps/$(MODULE)-*); \
	kcov --verify --include-path=$$dir/src $$target_dir $${file[0]}; \
	grep 'index.html' $$target_dir/index.js* | \
		grep --only-matching --extended-regexp \
		'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'
.PHONY: coverage\:lib

cov\:lib: coverage\:lib
.PHONY: cov\:lib

# NOTE:
# e2e requires also an actual application binary of 20min under the
# target/debug/deps directory.
coverage\:e2e: ## Generate a coverage report of e2e tests [alias: cov:e2e]
	set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/e2e"; \
	export CARGO_TARGET_DIR=$${target_dir}; \
	cargo test --test e2e --no-run --target-dir=$${target_dir}; \
	result=($${target_dir}/index.js*); \
	if [ -f $${result}[0] ]; then \
		rm "$${target_dir}/index.js*"; \
	fi; \
	file=($$target_dir/debug/deps/e2e-*); \
	kcov --verify --include-path=$$dir/src $$target_dir $${file[0]}; \
	grep 'index.html' $$target_dir/index.js* | \
		grep --only-matching --extended-regexp \
		'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'
.PHONY: coverage\:e2e

cov\:e2e: coverage\:e2e
.PHONY: cov\:e2e

coverage\:all: coverage\:lib coverage\:bin coverage\:e2e ## Generated merged coverage report of all tests
	set -uo pipefail; \
	dir="$$(pwd)"; \
	output_dir="$${dir}/target/coverage"; \
	kcov --merge $${output_dir} $$output_dir/lib $$output_dir/bin $$output_dir/e2e; \
	grep '\[merged\]' $$output_dir/index.js* | \
		grep --only-matching --extended-regexp \
		'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'
.PHONY: coverage\:all

coverage: coverage\:lib ## Same as coverage:lib [alias: cov]
.PHONY: coverage

cov: coverage
.PHONY: cov

# documentation
document: ## Generate documentation files [alias: doc]
	cargo rustdoc --package $(PACKAGE)
.PHONY: document

doc: document
.PHONY: doc
# }}}

# build
build\:debug: ## Create debug build
	cargo build
.PHONY: build\:debug

build\:release: ## Create release build
	cargo build --release
.PHONY: build\:release

build: build\:debug ## Same as build:debug
.PHONY: build

# utility
clean: ## Clean up
	@cargo clean
.PHONY: clean

# NOTE:
# This depends on environment variables from .env.ci, and requires
# the gitlab-runner command.
runner-%: ## Run a CI job on local (on Docker)
	@set -uo pipefail; \
	job=$(subst runner-,,$@); \
	opt=""; \
	while read line; do \
		opt+=" --env $$(echo $$line | sed -E 's/^export //')"; \
	done < .env.ci; \
	gitlab-runner exec docker \
		--executor docker \
		--cache-dir /cache \
		--docker-volumes $$(pwd)/.cache/gitlab-runner:/cache \
		--docker-volumes /var/run/docker.sock:/var/run/docker.sock \
		$${opt} $${job}
.PHONY: runner

help: ## Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\%\:\\\-\ ]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' ## ' | \
		sed --expression='s/\( [0-9a-z\:\\ ]*\) #/ #/' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": ## "}; \
			{printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = test
default: test
