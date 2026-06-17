import time
from dataclasses import dataclass
from typing import Any

import allure
import requests

from config.headers import DEFAULT_HEADERS


@dataclass(frozen=True)
class ApiResult:
    data: Any
    response: requests.Response


class BaseApiService:
    def __init__(self, base_url: str, session: requests.Session | None = None):
        self.base_url = base_url.rstrip("/")
        self.session = session or requests.Session()
        self.session.headers.update(DEFAULT_HEADERS)

    def get(self, path: str) -> ApiResult:
        url = f"{self.base_url}{path}"
        with allure.step(f"GET {path}"):
            response = self.session.get(url, timeout=10)
        return self._success_payload(response)

    def wait_for_health(self, timeout_seconds: int = 90) -> None:
        deadline = time.monotonic() + timeout_seconds
        last_error = None

        while time.monotonic() < deadline:
            try:
                response = self.session.get(f"{self.base_url}/health", timeout=5)
                if response.ok and response.json().get("success") is True:
                    return
            except Exception as error:
                last_error = str(error)
            time.sleep(1)

        raise AssertionError(f"backend health check did not pass: {last_error}")

    @staticmethod
    def _success_payload(response: requests.Response) -> ApiResult:
        assert response.ok, response.text
        payload = response.json()
        assert payload["success"] is True
        assert payload["error"] is None
        return ApiResult(data=payload["data"], response=response)
