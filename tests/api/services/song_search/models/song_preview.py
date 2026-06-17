from pydantic import BaseModel


class SongPreview(BaseModel):
    id: int
    description: str
    music_info: str
