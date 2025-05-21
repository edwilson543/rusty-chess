.PHONY:run
run:
	npm run dev


.PHONY:install
install:
	npm install
	npx playwright install chromium --with-deps


.PHONY:local_ci
local_ci: lint test playwright


.PHONY:test
test:
	npm test


.PHONY:playwright
playwright:
	npx playwright test

.PHONY:lint
lint:
	npm run lint
	npm run format


.PHONY:format
format:
	npm run format
