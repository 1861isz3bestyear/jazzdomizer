import os
import re
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path
from urllib.parse import quote

import psycopg
import pytest
from playwright.sync_api import sync_playwright


def pytest_configure(config):
    artifact_dir = Path(
        os.getenv(
            "API_TEST_ARTIFACT_DIR",
            Path(__file__).resolve().parents[1] / "artifacts",
        )
    )
    artifact_dir.mkdir(parents=True, exist_ok=True)
    config._api_artifact_dir = artifact_dir
    config._api_artifacts = []
    config._api_outcomes = []


@pytest.hookimpl(hookwrapper=True)
def pytest_runtest_makereport(item, call):
    outcome = yield
    report = outcome.get_result()
    if report.when == "call":
        item.config._api_outcomes.append(
            {
                "test": item.nodeid,
                "outcome": report.outcome,
                "duration_seconds": report.duration,
            }
        )


def pytest_sessionfinish(session, exitstatus):
    artifact = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "exitstatus": exitstatus,
        "api_base_url": os.getenv("API_BASE_URL", "http://localhost:3000/api"),
        "records": session.config._api_artifacts,
        "outcomes": session.config._api_outcomes,
    }

    artifact_path = session.config._api_artifact_dir / "api-test-artifact.json"
    artifact_path.write_text(
        __import__("json").dumps(artifact, indent=2, ensure_ascii=False),
        encoding="utf-8",
    )


@pytest.fixture(scope="session")
def api_base_url():
    return os.getenv("API_BASE_URL", "http://localhost:3000/api").rstrip("/")


@pytest.fixture(scope="session")
def database_url():
    value = os.getenv("DATABASE_URL")
    if not value:
        pytest.skip("DATABASE_URL is required for API relationship tests")
    return value


@pytest.fixture(scope="session")
def run_id():
    return os.getenv("API_TEST_RUN_ID", uuid.uuid4().hex[:10])


@pytest.fixture(scope="session")
def api_request_context(api_base_url):
    with sync_playwright() as playwright:
        request_context = playwright.request.new_context(
            extra_http_headers={"Accept": "application/json"},
        )
        wait_for_backend_health(request_context, api_base_url)
        yield request_context
        request_context.dispose()


def wait_for_backend_health(request_context, api_base_url, timeout_seconds=90):
    deadline = time.monotonic() + timeout_seconds
    last_error = None

    while time.monotonic() < deadline:
        try:
            response = request_context.get(f"{api_base_url}/health", timeout=5_000)
            if response.ok and response.json().get("success") is True:
                return
            last_error = f"health returned HTTP {response.status}"
        except Exception as error:
            last_error = str(error)
        time.sleep(1)

    raise AssertionError(f"backend health check did not pass: {last_error}")


@pytest.fixture
def db_conn(database_url):
    with psycopg.connect(database_url) as conn:
        yield conn


@pytest.fixture
def record_artifact(request):
    def record(name, data):
        request.config._api_artifacts.append(
            {
                "test": request.node.nodeid,
                "name": name,
                "data": data,
            }
        )

    return record


@pytest.fixture
def isolated_song_factory(db_conn, run_id, request, record_artifact):
    created_song_ids = []
    safe_test_name = re.sub(r"[^a-zA-Z0-9]+", "_", request.node.name).strip("_")
    prefix = f"api_pytest_{run_id}_{safe_test_name}_{uuid.uuid4().hex[:8]}"

    def create_song(*, name_suffix, version, chords, key, key_alteration=0, complexity=0):
        song_name = f"{prefix}_{name_suffix}"
        with db_conn.cursor() as cursor:
            cursor.execute(
                """
                insert into songs (song_name, song_version)
                values (%s, %s)
                returning id
                """,
                (song_name, version),
            )
            song_id = cursor.fetchone()[0]
            cursor.execute(
                """
                insert into harmony (song_id, chords, key)
                values (%s, %s, %s)
                """,
                (song_id, chords, key),
            )
            cursor.execute(
                """
                insert into list_data (song_id, key_alteration, complexity)
                values (%s, %s, %s)
                """,
                (song_id, key_alteration, complexity),
            )
        db_conn.commit()
        created_song_ids.append(song_id)
        record_artifact(
            "created_song",
            {
                "song_id": song_id,
                "song_name": song_name,
                "version": version,
                "key": key,
            },
        )
        return {
            "id": song_id,
            "name": song_name,
            "version": version,
            "chords": chords,
            "key": key,
        }

    yield create_song

    if created_song_ids:
        with db_conn.cursor() as cursor:
            cursor.execute("delete from songs where id = any(%s)", (created_song_ids,))
        db_conn.commit()
        record_artifact("deleted_song_ids", created_song_ids)


@pytest.fixture
def quote_path():
    return lambda value: quote(str(value), safe="")
