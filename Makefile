PACKAGE = cli-20min
BINARY = 20min

# verify
verify\:check:  ## Check rust syntax
	@cargo check --all -v
.PHONY: verify\:check

verify\:format:  ## Check format without changes [alias: verify:fmt, fmt]
	@cargo fmt --all -- --check
.PHONY: format

verify\:fmt: verify\:format
.PHONY: verify\:fmt

format: verify\:format
.PHONY: format

fmt: verify\:format
.PHONY: fmt

verify\:lint:  ## Check code style using clippy [alias: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: verify\:lint
.PHONY: lint

verify\:all: verify\:check verify\:format verify\:lint  ## Check by all verify:xxx targets
.PHONY: verify\:all

verify: verify\:check  ## Same as verify:check
.PHONY: verify

# test
test\:unit:  ## Run only unit tests
	@cargo test --bin $(BINARY)
.PHONY: test\:unit

test\:e2e:  ## Run e2e tests only
	@cargo test --test e2e
.PHONY: test\:e2e

test\:all:  ## Run all test targets [alias test]
	@cargo test --tests
.PHONY: test\:all

test: test\:all  ## Same as test:all
.PHONY: test

# coverage
coverage\:unit:  ## Generate a coverage report of unit tests [alias: cov:unit]
	@cargo test --bin $(BINARY) --no-run
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	output_dir="$${dir}/target/coverage"; \
	target_dir="$${dir}/target/debug/deps"; \
	if [ -f "$${output_dir}/index.js" ]; then \
		rm "$${output_dir}/index.js"; \
	fi; \
	i=0; \
	for file in $$(ls $$target_dir/$(PACKAGE)-* | \
		grep --invert-match '\.d$$'); do \
		kcov --verify --include-path=$$dir/src $$output_dir-$$i $$file; \
	done; \
	grep 'index.html' $$output_dir-0/index.js* | \
		grep --only-matching --extended-regexp \
			'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | \
			sed -E 's/[a-z\:"]*//g'
.PHONY: coverage\:unit

cov\:unit: coverage\:unit
.PHONY: cov\:unit

coverage\:e2e:  ## Generate a coverage report of e2e tests [alias: cov:e2e]
	@cargo test --test e2e --no-run
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	output_dir="$${dir}/target/coverage"; \
	target_dir="$${dir}/target/debug/deps"; \
	if [ -f "$${output_dir}/index.js" ]; then \
		rm "$${output_dir}/index.js"; \
	fi; \
	i=0; \
	for file in $$(ls $$target_dir/$(PACKAGE)-* | \
		grep --invert-match '\.d$$'); do \
		kcov --verify --include-path=$$dir/src $$output_dir-$$i $$file; \
	done; \
	grep 'index.html' $$output_dir-0/index.js* | \
		grep --only-matching --extended-regexp \
			'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | \
			sed -E 's/[a-z\:"]*//g'
.PHONY: coverage\:e2e

cov\:e2e: coverage\:e2e
.PHONY: cov\:e2e

# coverage
coverage:  ## Generate merged coverage report of all tests [alias: cov]
	@cargo test --all-targets --no-run
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	output_dir="$${dir}/target/coverage"; \
	target_dir="$${dir}/target/debug/deps"; \
	if [ -f "$${output_dir}/index.js" ]; then \
		rm "$${output_dir}/index.js"; \
	fi; \
	i=0; \
	for file in $$(ls $$target_dir/$(PACKAGE)-* | \
		grep --invert-match '\.d$$'); do \
		kcov --verify --include-path=$$dir/src $$output_dir-$$i $$file; \
	done; \
	kcov --merge $$output_dir-$\*; \
	grep '\[merged\]' $$output_dir-0/index.js* | \
		grep --only-matching --extended-regexp \
			'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | \
			sed -E 's/[a-z\:"]*//g'
.PHONY: coverage

cov: coverage
.PHONY: cov

# documentation
document:  ## Generate documentation files [alias: doc]
	cargo rustdoc --package $(PACKAGE)
.PHONY: document

doc: document
.PHONY: doc
# }}}

# build
build\:debug:  ## Create debug build
	cargo build
.PHONY: build\:debug

build\:release:  ## Create release build
	cargo build --release
.PHONY: build\:release

build: build\:debug  ## Same as build:debug
.PHONY: build

# utility
clean:  ## Clean up
	@cargo clean
.PHONY: clean

runner-%:  ## Run a CI job on local (on Docker)
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

help:  ## Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\%\:\\\-]+:  ## ' \
		$(firstword $(MAKEFILE_LIST)) | \
		sed --expression='s/\(\s|\(\s[0-9a-z\:\\]*\)*\)  /  /' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ":  ## "}; \
		  {printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = test
default: test
