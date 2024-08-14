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
	createdb chess-test || true
	DATABASE_URL=postgres://postgres:postgres@localhost/chess-test diesel migration run
	DATABASE_URL=postgres://postgres:postgres@localhost/chess-test cargo test || true
	dropdb chess-test


.PHONY:lint
lint:
	cargo check
	cargo fmt --all --check


.PHONY:format
format:
	cargo fmt

# Diesel.

.PHONY:setup_db
setup_db:
	dropdb chess || true
	createdb chess
	diesel setup
	make migrate

.PHONY:migrate
migrate:
	diesel migration run
	diesel migration redo
