import json
import os
from datetime import datetime, timezone
from pathlib import Path

import pytest


def pytest_configure(config):
    artifact_dir = Path(
        os.getenv("API_TEST_ARTIFACT_DIR", Path(__file__).resolve().parent.parent / "artifacts")
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
    artifact_path.write_text(json.dumps(artifact, indent=2), encoding="utf-8")

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
