.PHONY: all audit build coverage format lint test

all: test lint build

audit:
	cargo audit

build:
	cargo build

coverage:
	cargo tarpaulin

format:
	cargo fmt -- --check

lint:
	cargo clippy -- -D warnings

test:
	cargo test

watch:
	cargo watch -x test
