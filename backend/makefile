.PHONY:run
run:
	cargo run


.PHONY:build
build:
	cargo build


.PHONY:local_ci
local_ci: test lint


.PHONY:test
test:
	cargo test


.PHONY:lint
lint:
	cargo check
	cargo fmt --all --check


.PHONY:format
format:
	cargo fmt
