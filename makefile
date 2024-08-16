.PHONY:runbe
runbe:
	cd backend && make run

.PHONY:runfe
runfe:
	cd frontend && make run


.PHONY:build
build:
	cd backend && make build
	cd frontend && make build


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
