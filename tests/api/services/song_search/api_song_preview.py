from urllib.parse import quote

from config.base_test import BaseApiService
from services.song_search.endpoints import SONGS, SONG_SEARCH
from services.song_search.models.song_preview import SongPreview


class SongPreviewApi(BaseApiService):
    def search(self, term: str) -> list[SongPreview]:
        path = SONG_SEARCH.format(term=quote(str(term), safe=""))
        result = self.get(path)
        return [SongPreview.model_validate(item) for item in result.data]

    def list(self) -> list[SongPreview]:
        result = self.get(SONGS)
        return [SongPreview.model_validate(item) for item in result.data]
