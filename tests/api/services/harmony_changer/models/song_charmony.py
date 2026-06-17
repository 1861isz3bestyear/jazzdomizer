from pydantic import BaseModel


class SongCharmony(BaseModel):
    id: int
    description: str
    music_info: str
