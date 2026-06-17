# API relationship tests

This directory contains Python API tests for the relationship between Postgres data and the Rust backend API.

The tests use:

- `pytest` as the test runner.
- `requests` for API HTTP requests.
- `pydantic` for response validation.
- `allure-pytest` for test annotations and optional Allure reports.
- `psycopg` for direct database setup and verification.
- `uv` for dependency management.

The suite creates uniquely named database rows per test, verifies that backend endpoints reflect those rows, and deletes the rows afterward. This keeps tests isolated and suitable for future parallel execution.

Generated artifacts:

- `api-test-artifact.json`: structured summary of created rows, checked endpoints, and observed API/DB values.
- `junit.xml`: pytest JUnit report when run through Docker Compose.

Run with Docker Compose from the repository root:

```sh
docker compose -f docker/docker-compose.api-tests.yml up --build --abort-on-container-exit --exit-code-from api-tests
```

Artifacts are written to:

```text
test-artifacts/api/
```

Local run, when backend and Postgres are already available:

```sh
cd tests/api
uv sync
API_BASE_URL=http://localhost:3000/api \
DATABASE_URL=postgresql://jazzdomizer:jazzdomizer@localhost:5432/rust_sql_one_db \
uv run pytest
```
