from config.base_test import BaseApiService
from services.harmony_changer.endpoints import SONG_DETAIL, TRANSPOSE
from services.harmony_changer.models.song_charmony import SongCharmony


class HarmonyApi(BaseApiService):
    def detail(self, song_id: int) -> SongCharmony:
        result = self.get(SONG_DETAIL.format(song_id=song_id))
        return SongCharmony.model_validate(result.data)

    def transpose(self, song_id: int, steps: int) -> SongCharmony:
        result = self.get(TRANSPOSE.format(song_id=song_id, steps=steps))
        return SongCharmony.model_validate(result.data)
