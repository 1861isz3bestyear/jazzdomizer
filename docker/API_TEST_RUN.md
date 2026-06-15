# Docker API test run

Run API relationship tests from the repository root:

```sh
docker compose -f docker/docker-compose.api-tests.yml up --build --abort-on-container-exit --exit-code-from api-tests
```

After the run, artifacts are available on the host at:

```text
test-artifacts/api/
```

Expected files:

```text
test-artifacts/api/api-test-artifact.json
test-artifacts/api/junit.xml
```

`api-test-artifact.json` contains the created test rows, checked endpoints, observed API data, and cleanup IDs.

`junit.xml` is the pytest JUnit report for CI systems.

Clean up containers and the compose network after a run:

```sh
docker compose -f docker/docker-compose.api-tests.yml down
```

The compose file does not publish fixed host ports and does not set fixed container names. This keeps runs project-scoped and avoids collisions with the existing application compose stack.
