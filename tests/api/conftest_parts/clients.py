import pytest

from services.harmony_changer.api_harmony import HarmonyApi
from services.song_search.api_song_preview import SongPreviewApi


@pytest.fixture
def song_preview_api(api_base_url, api_session):
    return SongPreviewApi(api_base_url, api_session)


@pytest.fixture
def harmony_api(api_base_url, api_session):
    return HarmonyApi(api_base_url, api_session)
