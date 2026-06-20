import allure
from services.harmony_changer.payloads import DETAIL_SONG, TRANSPOSE_SONG
from services.song_search.payloads import SEARCH_SONG


@allure.feature("Search song preview by name")
def test_search_song_preview_by_name(
    song_preview_api,
    isolated_song_factory,
    record_artifact,
):
    song = isolated_song_factory(**SEARCH_SONG)
    songs = song_preview_api.search(song["name"])
    matching_song = next((item for item in songs if item.id == song["id"]), None)
    assert matching_song is not None
    assert matching_song.description == song["name"]
    assert matching_song.music_info == SEARCH_SONG["key"]
    record_artifact("search_result", matching_song.model_dump())

@allure.feature("Search song detail by id")
def test_search_song_detail_by_id(
        harmony_api,
        db_conn,
        isolated_song_factory,
        record_artifact,
):
    detail_song = isolated_song_factory(**DETAIL_SONG)
    detail = harmony_api.detail(detail_song["id"])
    assert detail.id == detail_song["id"]
    assert detail.description == f"{detail_song['name']} - relationship"
    assert "A)" in detail.music_info
    assert "Cmaj7/G" in detail.music_info
    record_artifact("search_result", detail_song)

@allure.feature("Transpose song")
def test_transpose_song(
        harmony_api,
        db_conn,
        isolated_song_factory,
        record_artifact,
):
    transpose_song = isolated_song_factory(**TRANSPOSE_SONG)
    transposed = harmony_api.transpose(transpose_song["id"], steps=2)
    assert transposed.id == transpose_song["id"]
    assert "Dmaj7/A" in transposed.music_info
    assert "Bm7" in transposed.music_info
    with db_conn.cursor() as cursor:
         cursor.execute("select chords from harmony where song_id = %s", (transpose_song["id"],))
         stored_chords = cursor.fetchone()[0]
    unchanged_detail = harmony_api.detail(transpose_song["id"])
    assert stored_chords == TRANSPOSE_SONG["chords"]
    assert "Cmaj7/G" in unchanged_detail.music_info
    assert "Dmaj7/A" not in unchanged_detail.music_info
#     record_artifact(
#         "harmony_result",
#         {
#             "detail": detail.model_dump(),
#             "transposed_music_info": transposed.music_info,
#             "stored_chords_after_request": stored_chords,
#         },
# @allure.feature("Database and backend relationship")
# def test_song_detail_and_transpose_read_harmony_without_mutating_database(
#     harmony_api,
#     db_conn,
#     isolated_song_factory,
#     record_artifact,
# ):
#     detail_song = isolated_song_factory(**DETAIL_SONG)
#     detail = harmony_api.detail(detail_song["id"])
#     assert detail.id == detail_song["id"]
#     assert detail.description == f"{detail_song['name']} - relationship"
#     assert "A)" in detail.music_info
#     assert "B)" in detail.music_info
#     assert "Cmaj7/G" in detail.music_info

#     transpose_song = isolated_song_factory(**TRANSPOSE_SONG)
#     transposed = harmony_api.transpose(transpose_song["id"], steps=2)
#     assert transposed.id == transpose_song["id"]
#     assert "Dmaj7/A" in transposed.music_info
#     assert "Bm7" in transposed.music_info
#     with db_conn.cursor() as cursor:
#         cursor.execute("select chords from harmony where song_id = %s", (transpose_song["id"],))
#         stored_chords = cursor.fetchone()[0]
#     unchanged_detail = harmony_api.detail(transpose_song["id"])
#     assert stored_chords == TRANSPOSE_SONG["chords"]
#     assert "Cmaj7/G" in unchanged_detail.music_info
#     assert "Dmaj7/A" not in unchanged_detail.music_info
#     record_artifact(
#         "harmony_result",
#         {
#             "detail": detail.model_dump(),
#             "transposed_music_info": transposed.music_info,
#             "stored_chords_after_request": stored_chords,
#         },
#     )
