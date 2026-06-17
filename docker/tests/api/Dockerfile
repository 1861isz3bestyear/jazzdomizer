FROM python:3.12-slim

COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /usr/local/bin/

WORKDIR /workspace/tests/api

ENV API_TEST_ARTIFACT_DIR=/artifacts
ENV PYTHONUNBUFFERED=1
ENV UV_LINK_MODE=copy

COPY tests/api/pyproject.toml tests/api/uv.lock tests/api/README.md ./
COPY tests/api/conftest_parts ./conftest_parts
COPY tests/api/conftest.py ./
COPY tests/api/config ./config
COPY tests/api/services ./services
COPY tests/api/tests ./tests
COPY tests/api/utils ./utils

RUN uv sync --locked --no-install-project

CMD ["uv", "run", "pytest", "-q", "--junitxml=/artifacts/junit.xml"]
