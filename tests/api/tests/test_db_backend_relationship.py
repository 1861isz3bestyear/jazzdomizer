def assert_success_payload(response):
    assert response.ok, response.text()
    payload = response.json()
    assert payload["success"] is True
    assert payload["error"] is None
    return payload["data"]


def test_search_endpoint_reflects_song_inserted_in_database(
    api_request_context,
    api_base_url,
    isolated_song_factory,
    quote_path,
    record_artifact,
):
    song = isolated_song_factory(
        name_suffix="search",
        version="pytest",
        chords="A) |Cmaj7|F7|",
        key="C",
    )

    endpoint = f"{api_base_url}/songs/search/{quote_path(song['name'])}"
    response = api_request_context.get(endpoint)
    songs = assert_success_payload(response)
    matching_song = next((item for item in songs if item["id"] == song["id"]), None)

    assert matching_song is not None
    assert matching_song["description"] == song["name"]
    assert matching_song["music_info"] == "C"

    record_artifact(
        "search_result",
        {
            "endpoint": endpoint,
            "matched_song": matching_song,
        },
    )


def test_song_detail_endpoint_reads_description_and_harmony_from_database(
    api_request_context,
    api_base_url,
    isolated_song_factory,
    record_artifact,
):
    song = isolated_song_factory(
        name_suffix="detail",
        version="relationship",
        chords="A) |Cmaj7/G|Am7|\nB) |Dm7|G7|",
        key="C",
    )

    endpoint = f"{api_base_url}/songs/{song['id']}"
    response = api_request_context.get(endpoint)
    detail = assert_success_payload(response)

    assert detail["id"] == song["id"]
    assert detail["description"] == f"{song['name']} - relationship"
    assert "A)" in detail["music_info"]
    assert "B)" in detail["music_info"]
    assert "Cmaj7/G" in detail["music_info"]

    record_artifact(
        "detail_result",
        {
            "endpoint": endpoint,
            "detail": detail,
        },
    )


def test_transpose_endpoint_uses_database_harmony_and_does_not_mutate_source_row(
    api_request_context,
    api_base_url,
    db_conn,
    isolated_song_factory,
    record_artifact,
):
    source_chords = "A) |Cmaj7/G|Am7|Dm7|G7|"
    song = isolated_song_factory(
        name_suffix="transpose",
        version="immutable",
        chords=source_chords,
        key="C",
    )

    transpose_endpoint = f"{api_base_url}/songs/{song['id']}/transpose/2"
    response = api_request_context.get(transpose_endpoint)
    transposed = assert_success_payload(response)

    assert transposed["id"] == song["id"]
    assert "Dmaj7/A" in transposed["music_info"]
    assert "Bm7" in transposed["music_info"]

    with db_conn.cursor() as cursor:
        cursor.execute("select chords from harmony where song_id = %s", (song["id"],))
        stored_chords = cursor.fetchone()[0]

    detail_response = api_request_context.get(f"{api_base_url}/songs/{song['id']}")
    detail = assert_success_payload(detail_response)

    assert stored_chords == source_chords
    assert "Cmaj7/G" in detail["music_info"]
    assert "Dmaj7/A" not in detail["music_info"]

    record_artifact(
        "transpose_result",
        {
            "endpoint": transpose_endpoint,
            "transposed_music_info": transposed["music_info"],
            "stored_chords_after_request": stored_chords,
        },
    )


def test_song_list_endpoint_includes_database_inserted_song(
    api_request_context,
    api_base_url,
    isolated_song_factory,
    record_artifact,
):
    song = isolated_song_factory(
        name_suffix="list",
        version="visible",
        chords="A) |Fmaj7|Bb7|",
        key="F",
    )

    endpoint = f"{api_base_url}/songs"
    response = api_request_context.get(endpoint)
    songs = assert_success_payload(response)
    matching_song = next((item for item in songs if item["id"] == song["id"]), None)

    assert matching_song is not None
    assert matching_song["description"] == song["name"]
    assert matching_song["music_info"] == "F"

    record_artifact(
        "list_result",
        {
            "endpoint": endpoint,
            "matched_song": matching_song,
            "returned_count": len(songs),
        },
    )
