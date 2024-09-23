run: run_backend run_frontend

.PHONY:run_backend
run_backend:
	cd backend && make run

.PHONY:run_frontend
run_frontend:
	cd frontend && make run


.PHONY:install
install:
	curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
	cd backend && make setup_db
	cd backend && make build
	cd frontend && make install


.PHONY:local_ci
local_ci:
	cd backend && make local_ci
	cd frontend && make local_ci


.PHONY:test
test:
	cd backend && make test
	cd frontend && make test


.PHONY:lint
lint:
	cd backend && make lint
	cd frontend && make lint


.PHONY:format
format:
	cd backend && make format
	cd frontend && make format
