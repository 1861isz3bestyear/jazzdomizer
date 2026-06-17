# API tests

These tests verify the relationship between the database and the backend API.

The test flow is:

1. Create an isolated song row directly in Postgres.
2. Create matching `harmony` and `list_data` rows.
3. Call the backend API with `requests` service clients.
4. Assert the API response reflects the database row.
5. Delete only the rows created by that test.
6. Write structured evidence into `api-test-artifact.json`.

Covered behavior:

- `/api/songs/search/{term}` returns a song inserted directly into the DB.
- `/api/songs/{id}` reads song description and harmony from DB rows.
- `/api/songs/{id}/transpose/{steps}` transposes harmony from DB data without mutating the stored row.

Isolation model:

- Each test creates rows with a unique `api_pytest_<run_id>_<test_name>_<uuid>` prefix.
- Tests track their inserted `song_id` values.
- Cleanup deletes from `songs`; the schema cascades related `harmony` rows.
- No test depends on another test's inserted data.

Python dependencies are managed by `uv` in `tests/api/pyproject.toml` and locked in `tests/api/uv.lock`.
