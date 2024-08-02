.PHONY:runbe
runbe:
	cd backend && cargo run

.PHONY:runfe
runfe:
	cd frontend && npm run dev


.PHONY:build
build:
	cd backend && cargo build
	cd frontend && npm run build


.PHONY:local_ci
local_ci: test lint


.PHONY:test
test:
	cd backend && cargo test
	cd frontend && npm test


.PHONY:lint
lint:
	cd backend && cargo check
	cd backend && cargo fmt --all --check
	cd frontend && npm run lint
	cd frontend && npm run format


.PHONY:format
format:
	cd backend && cargo fmt
	cd frontend && npm run format
