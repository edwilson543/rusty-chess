.PHONY:run
run:
	cd backend && cargo run


.PHONY:build
build:
	cd backend && cargo build


.PHONY:local_ci
local_ci: test lint


.PHONY:test
test:
	cd backend && cargo test


.PHONY:lint
lint:
	cd backend && cargo check
	cd backend && cargo fmt --all --check


.PHONY:format
format:
	cd backend && cargo fmt
