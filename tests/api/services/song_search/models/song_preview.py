from pydantic import BaseModel, field_validator


class SongPreview(BaseModel):
    id: int
    description: str
    music_info: str

    @field_validator("id", "description", "music_info")
    def fields_not_empty(cls, value):
        if value is None or value == "":
            raise ValueError("Field is empty")
        return value
