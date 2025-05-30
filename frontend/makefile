.PHONY:run
run:
	npm run dev


.PHONY:install
install:
	npm install
	npx playwright install chromium --with-deps


.PHONY:local_ci
local_ci: lint depcruiser test playwright


.PHONY:test
test:
	npm test


.PHONY:playwright
playwright:
	docker compose -f ../backend/docker-compose.yaml up -d
	npx playwright test
	docker compose -f ../backend/docker-compose.yaml down

.PHONY:lint
lint:
	npm run lint
	npm run format

.PHONY:depcruiser
depcruiser:
	npm run depcruiser

.PHONY:format
format:
	npm run format
