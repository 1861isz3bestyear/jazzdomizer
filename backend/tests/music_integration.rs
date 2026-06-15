use jazzdomizer_backend::{
    db::SongMusicInfo,
    music_logic::{ChordForReading, Key, Note, Song, SongHarmony},
};

#[test]
fn song_from_database_record_parses_key_and_multisection_harmony() {
    let song = Song::from_db(SongMusicInfo {
        song_id: 42,
        key: "Dbm".to_string(),
        chords: r#"
            Intro)
            |Dbm9|Gb7|Cbmaj7|Fb|
            Bridge)
            |Abm Db7|Gbmaj7 Cbmaj7|
        "#
        .to_string(),
    })
    .expect("database song should parse");

    assert_eq!(song.id, 42);
    assert_eq!(song.key.to_string(), "C#m");
    assert_eq!(song.harmony.parts.len(), 2);
    assert_eq!(song.harmony.parts[0].name, "Intro");
    assert_eq!(song.harmony.parts[0].bars.len(), 4);
    assert_eq!(song.harmony.parts[1].bars[0].chords.len(), 2);
    assert_eq!(song.harmony.parts[1].bars[0].chords[1].to_string(), "C#7");
}

#[test]
fn transposing_song_updates_key_roots_and_slash_bass_notes_together() {
    let song = Song::from_db(SongMusicInfo {
        song_id: 7,
        key: "C".to_string(),
        chords: "Verse)\n|Cmaj7/G|Am7|Dm7|G7|".to_string(),
    })
    .expect("database song should parse");

    let transposed = song.alter_by_steps(2);

    assert_eq!(transposed.key.to_string(), "D");
    assert_eq!(
        transposed.harmony.parts[0].bars[0].chords[0],
        ChordForReading {
            root: Note::DNat,
            color: Some("maj7".to_string()),
            bass: Some(Note::ANat),
        }
    );
    assert_eq!(
        transposed.harmony.parts[0]
            .bars
            .iter()
            .map(|bar| bar.chords[0].to_string())
            .collect::<Vec<_>>(),
        vec!["Dmaj7/A", "Bm7", "Em7", "A7"]
    );
}

#[test]
fn parsed_harmony_display_preserves_sections_and_bar_layout() {
    let harmony = SongHarmony::from_db_string(
        r#"
        A)
        |C|Am|Dm|G|
        B)
        |Fmaj7 C/E|Dm7 G7|
        "#,
    )
    .expect("harmony should parse");

    let rendered = harmony.to_string();

    assert!(rendered.contains("A)"));
    assert!(rendered.contains("B)"));
    assert!(rendered.contains("|C"));
    assert!(rendered.contains("Fmaj7"));
    assert!(rendered.contains("C/E"));
}

#[test]
fn key_and_chord_parsing_share_enharmonic_normalization() {
    let key = Key::from_db_string("G#m").expect("key should parse");
    let chord = ChordForReading::from_db_string("Dbmaj7/F")
        .expect("chord should parse");

    assert_eq!(key.to_string(), "Abm");
    assert_eq!(chord.root, Note::CSharp);
    assert_eq!(chord.bass, Some(Note::FNat));
    assert_eq!(chord.to_string(), "C#maj7/F");
}
