.PHONY: all build test dev bench format clean examples

all: format build test examples

build:
	cargo build

test:
	cargo test -- --nocapture

dev: format test

bench: format
	cargo bench -- --nocapture

format:
	@cargo fmt --all -- --check >/dev/null || cargo fmt --all

clean:
	cargo clean

examples:
	cargo build --example example_usage