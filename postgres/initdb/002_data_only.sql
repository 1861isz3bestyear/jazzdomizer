--
-- Data dump for local database rust_sql_one_db.
-- Generated from the local machine data and kept column-explicit so it can be
-- loaded by the Docker Postgres init process after the copied schema file.
--

truncate table list_data, harmony, songs restart identity cascade;

insert into public.songs (id, song_name, song_version) values
    (1, 'Blues', '12 bars major blues'),
    (2, 'Blue bossa', ''),
    (3, 'All the things you are', 'Ella Fitzgerald'),
    (4, 'There will never be another you', ''),
    (5, 'Countdown', 'John Coltrane'),
    (6, 'Teen town', 'Pastorius, Weather report'),
    (7, 'So what', 'Miles Davis'),
    (8, 'Giant steps', 'John Coltrane');

insert into public.harmony (id, song_id, chords, key) values
    (1, 1, 'A) |C7|F7|C7|Gm7 C7|F7|F#º7|C7|Em7 A7|Dm7|G7|C7 A7|Dm7 G7|', 'C'),
    (3, 2, 'A) |Cm7|Cm7|Fm7|Fm7|
|Dø7| G7b9|Cm7|Cm7|
|Ebm7|Ab7|Dbmaj|Dbmaj|
|Dø7|G7b9|Cm7|Dø7 G7b9|
', 'Cm'),
    (2, 3, 'A) |Fm7|Bbm7|Eb7 |Abmaj|Dbmaj|Dm7 A7|Cmaj|Cmaj|
|Cm7|Fm7|Bb7|Ebmaj|Abmaj|Am7 D7|Gmaj|Gmaj|
B) |Am7|D7|Gmaj|Gmaj|F#ø7|B7b9|Emaj|C7b13
A) |Fm7|Bbm7|Eb7 |Abmaj|Dbmaj|Dbmaj-7|Cm7|Bº7|Bbm7|Eb7|Abmaj|Gø7 C7b9|', 'Fm'),
    (4, 4, 'A) |Ebmaj|Ebmaj|Dø7|G7b9|
|Cm7|Cm7|Bbm7|Eb7|
B) |Abmaj|Db7#11|Ebmaj|Cm7|
|F7|F7|Fm7|Bb7|
A) |Ebmaj|Ebmaj|Dø7|G7b9|
|Cm7|Cm7|Bbm7|Eb7|
B) |Abmaj|Db7#11|Ebmaj|Aø7 D7b9|
|Gm7 C7|Fm7 Bb7|Ebmaj|Bb7|
', 'Eb'),
    (7, 5, '
	A)
	|Em7 F7|Bbmaj7 Db7|Gbmaj7 A7|Dmaj7 |
	|Dm7 Eb7|Abmaj7 B7|Emaj7 G7|Cmaj7 |
	|Cm7 Db7|Gbmaj7 A7|Dmaj7 F7|Bbmaj7 |
	|Em7|F7|Bbmaj7|A7| 
	Segno) 
	|Em7 F7|Bbmaj7 Dbmaj7|Gbmaj7 F7|Bbmaj7 A7|
	|Dmaj7 Bbmaj7|Gbmaj7 Dmaj7|Bbmaj7 Gbmaj7|Dmaj7|
	', 'Bb'),
    (8, 6, 'A) |C7|A7|F7|D7|', 'C'),
    (9, 7, 'A) |Dm7|Dm7|Dm7|Dm7|
|Dm7|Dm7|Dm7|Dm7|
|Ebm7|Ebm7|Ebm7|Ebm7|
Dm7|Dm7|Dm7|Dm7|', 'Dm'),
    (10, 8, 'A) |Bmaj7 D7|Gmaj7 Bb7|Ebmaj7|Am7 D7|
|Gmaj7 Bb7|Ebmaj7 Gb7|Bmaj7|Fm7 Bb7|
|Ebmaj7|Am7 D7|Gmaj7|Dbm7 Gb7|
|Bmaj7|Fm7 Bb7|Ebmaj7|Dbm7 Gb7|
', 'Eb');

insert into public.list_data (
    id,
    song_id,
    key_alteration,
    complexity,
    global_playlist,
    local_playlist,
    song_for_study
) values
    (8, 8, 4, 0, false, false, true),
    (4, 4, 0, 6, false, false, false),
    (2, 2, 0, 10, false, false, false),
    (3, 3, 1, 11, false, false, false),
    (1, 1, 0, 1, false, false, false),
    (6, 6, 4, 0, true, true, false),
    (7, 7, 1, 0, true, true, false),
    (5, 5, 4, 0, true, true, false);

select pg_catalog.setval('public.songs_id_seq', 9, true);
select pg_catalog.setval('public.harmony_id_seq', 10, true);
select pg_catalog.setval('public.list_data_id_seq', 8, true);
