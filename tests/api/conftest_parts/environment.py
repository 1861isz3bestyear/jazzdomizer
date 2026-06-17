import os
import uuid

import pytest
import requests

from config.base_test import BaseApiService


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
def api_session(api_base_url, database_url):
    with requests.Session() as session:
        BaseApiService(api_base_url, session).wait_for_health()
        yield session
