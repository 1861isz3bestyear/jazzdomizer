import pytest

from utils.helpers import unique_song_prefix


@pytest.fixture
def isolated_song_factory(db_conn, run_id, request, record_artifact):
    created_song_ids = []
    prefix = unique_song_prefix(run_id, request.node.name)

    def create_song(*, name_suffix, version, chords, key, key_alteration=0, complexity=0):
        song_name = f"{prefix}_{name_suffix}"
        with db_conn.cursor() as cursor:
            cursor.execute(
                "insert into songs (song_name, song_version) values (%s, %s) returning id",
                (song_name, version),
            )
            song_id = cursor.fetchone()[0]
            cursor.execute(
                "insert into harmony (song_id, chords, key) values (%s, %s, %s)",
                (song_id, chords, key),
            )
            cursor.execute(
                """
                insert into list_data (song_id, key_alteration, complexity)
                values (%s, %s, %s)
                """,
                (song_id, key_alteration, complexity),
            )
        db_conn.commit()
        created_song_ids.append(song_id)
        record_artifact(
            "created_song",
            {"song_id": song_id, "song_name": song_name, "version": version, "key": key},
        )
        return {"id": song_id, "name": song_name, "version": version, "chords": chords}

    yield create_song

    if created_song_ids:
        with db_conn.cursor() as cursor:
            cursor.execute("delete from songs where id = any(%s)", (created_song_ids,))
        db_conn.commit()
        record_artifact("deleted_song_ids", created_song_ids)
