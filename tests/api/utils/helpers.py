import re
import uuid


def safe_name(value: str) -> str:
    return re.sub(r"[^a-zA-Z0-9]+", "_", value).strip("_")


def unique_song_prefix(run_id: str, test_name: str) -> str:
    return f"api_pytest_{run_id}_{safe_name(test_name)}_{uuid.uuid4().hex[:8]}"
