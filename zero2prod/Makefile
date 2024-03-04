.PHONY: all audit build coverage deps format lint run test watch

all: format test lint build

audit:
	cargo audit

build:
	cargo build

coverage:
	cargo tarpaulin

deps:
	cargo update
	cargo +nightly udeps

format:
	cargo fmt -- --check

lint:
	cargo clippy -- -D warnings

run:
	cargo run

test:
	cargo test

watch:
	cargo watch -x test
