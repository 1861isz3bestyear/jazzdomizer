use crate::db::{
    PgPool, SongDescription, SongFullInfoRaw, SongListInfo, SongMusicInfo, SongPreviewRaw,
};
use crate::music_logic::{Key, SongHarmony};
use std::fmt;

#[derive(Debug, Clone)]
pub struct SongFullInfo {
    pub id: i32,
    pub description: String,
    pub music_info: SongHarmony,
}

impl SongFullInfo {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        Ok(Self::from_raw_songs(SongFullInfoRaw::get_all(pool).await?))
    }

    pub async fn get_songs_by_name(
        pool: &PgPool,
        search_term: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(Self::from_raw_songs(
            SongFullInfoRaw::get_songs_by_part_name(pool, search_term).await?,
        ))
    }

    pub async fn show_default(pool: &PgPool, song_id: i32) -> Result<Self, sqlx::Error> {
        let db_description = SongDescription::get_song_description_by_id(pool, song_id).await?;
        let db_music_info = SongMusicInfo::get_song_music_info(pool, song_id).await?;
        Self::from_default_parts(song_id, db_description, db_music_info)
    }

    pub async fn alter_by_db(pool: &PgPool, song_id: i32) -> Result<Self, sqlx::Error> {
        let db_description = SongListInfo::get_song_list_info(pool, song_id).await?;
        let def = SongFullInfo::show_default(pool, song_id).await?;
        let steps = Self::steps_or_default(song_id, db_description.as_ref());
        Ok(Self::from_altered_default(def, steps))
    }

    pub async fn alter_by_steps(&self, steps: i32) -> Result<Self, sqlx::Error> {
        Ok(Self::from_altered_default(self.clone(), steps))
    }

    fn from_raw_songs(raw_songs: Vec<SongFullInfoRaw>) -> Vec<Self> {
        raw_songs
            .into_iter()
            .filter_map(|raw| {
                let music_info = SongHarmony::from_db_string(&raw.harmony).ok()?;
                Some(SongFullInfo {
                    id: raw.song_id,
                    description: raw.song_description,
                    music_info,
                })
            })
            .collect()
    }

    fn from_default_parts(
        song_id: i32,
        db_description: Option<SongDescription>,
        db_music_info: Option<SongMusicInfo>,
    ) -> Result<Self, sqlx::Error> {
        let description = db_description
            .as_ref()
            .map(|desc| match &desc.song_version {
                Some(version) => format!("{} - {}", desc.song_name, version),
                None => desc.song_name.clone(),
            })
            .unwrap_or_else(|| format!("Song with id {} not found", song_id));
        let harmony = db_music_info
            .as_ref()
            .map(|info| info.chords.clone())
            .unwrap_or_else(|| "No harmony data".to_string());
        let parsed_harmony = SongHarmony::from_db_string(&harmony)
            .map_err(|e| sqlx::Error::Protocol(format!("Harmony can't be parsed: {}", e)))?;

        Ok(SongFullInfo {
            id: song_id,
            description,
            music_info: parsed_harmony,
        })
    }

    fn steps_or_default(song_id: i32, db_description: Option<&SongListInfo>) -> i32 {
        db_description
            .map(|info| info.key_alteration)
            .unwrap_or_else(|| {
                eprintln!("No alteration data for song with id {}", song_id);
                0
            })
    }

    fn from_altered_default(song: Self, steps: i32) -> Self {
        Self {
            id: song.id,
            description: song.description,
            music_info: song.music_info.alter_by_steps(steps),
        }
    }
}

impl fmt::Display for SongFullInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}\n{}", self.description, self.music_info)
    }
}

#[derive(Debug, Clone)]
pub struct SongPreviewInfo {
    pub id: i32,
    pub description: String,
    pub music_info: Key,
}

impl SongPreviewInfo {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        Ok(Self::from_raw_songs(SongPreviewRaw::get_all(pool).await?))
    }

    pub async fn get_songs_by_name(
        pool: &PgPool,
        search_term: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(Self::from_raw_songs(
            SongPreviewRaw::get_songs_by_part_name(pool, search_term).await?,
        ))
    }

    pub async fn show_default(pool: &PgPool, song_id: i32) -> Result<Self, sqlx::Error> {
        let db_description = SongDescription::get_song_description_by_id(pool, song_id).await?;
        let db_music_info = SongMusicInfo::get_song_music_info(pool, song_id).await?;
        Self::from_default_parts(song_id, db_description, db_music_info)
    }

    pub async fn alter_by_db(&self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let db_description = SongListInfo::get_song_list_info(pool, self.id).await?;
        let steps = Self::steps_or_default(self.id, db_description.as_ref());
        Ok(Self::from_altered_default(self.clone(), steps))
    }

    pub async fn alter_by_steps(&self, steps: i32) -> Result<Self, sqlx::Error> {
        Ok(Self::from_altered_default(self.clone(), steps))
    }

    fn from_raw_songs(raw_songs: Vec<SongPreviewRaw>) -> Vec<Self> {
        raw_songs
            .into_iter()
            .filter_map(|raw| {
                let music_info = Key::from_db_string(&raw.harmony_key).ok()?;
                Some(SongPreviewInfo {
                    id: raw.song_id,
                    description: raw.song_name,
                    music_info,
                })
            })
            .collect()
    }

    fn from_default_parts(
        song_id: i32,
        db_description: Option<SongDescription>,
        db_music_info: Option<SongMusicInfo>,
    ) -> Result<Self, sqlx::Error> {
        let description = match db_description {
            Some(desc) => desc.song_name,
            None => format!("Song with id {} not found", song_id),
        };
        let key = match db_music_info {
            Some(info) => info.key,
            None => "No key data".to_string(),
        };
        let parsed_key = Key::from_db_string(&key)
            .map_err(|e| sqlx::Error::Protocol(format!("Key can't be parsed: {}", e)))?;
        Ok(SongPreviewInfo {
            id: song_id,
            description,
            music_info: parsed_key,
        })
    }

    fn steps_or_default(song_id: i32, db_description: Option<&SongListInfo>) -> i32 {
        db_description
            .map(|info| info.key_alteration)
            .unwrap_or_else(|| {
                eprintln!("No alteration data for song with id {}", song_id);
                0
            })
    }

    fn from_altered_default(song: Self, steps: i32) -> Self {
        Self {
            id: song.id,
            description: song.description,
            music_info: song.music_info.alter_by_steps(steps),
        }
    }
}

impl fmt::Display for SongPreviewInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{} - Key:{}", self.description, self.music_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music_logic::{Key, Mode, Note, SongHarmony};
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;

    fn harmony() -> SongHarmony {
        SongHarmony::from_db_string("A) |C/E|G7|").unwrap()
    }

    fn lazy_pool() -> PgPool {
        PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(25))
            .connect_lazy("postgres://invalid:invalid@127.0.0.1:1/invalid?connect_timeout=1")
            .unwrap()
    }

    #[tokio::test]
    async fn song_full_info_alter_by_steps_transposes_harmony() {
        let song = SongFullInfo {
            id: 9,
            description: "Tune".to_string(),
            music_info: harmony(),
        };

        let altered = song.alter_by_steps(1).await.unwrap();

        assert_eq!(altered.id, 9);
        assert_eq!(altered.description, "Tune");
        assert_eq!(altered.music_info.parts[0].bars[0].chords[0].to_string(), "G/B");
        assert_eq!(altered.music_info.parts[0].bars[1].chords[0].to_string(), "D7");
    }

    #[test]
    fn song_full_info_display_includes_description_and_harmony() {
        let song = SongFullInfo {
            id: 1,
            description: "Displayed Tune".to_string(),
            music_info: harmony(),
        };

        let rendered = song.to_string();
        assert!(rendered.contains("Displayed Tune"));
        assert!(rendered.contains("A)"));
        assert!(rendered.contains("C/E"));
    }

    #[tokio::test]
    async fn song_preview_info_alter_by_steps_transposes_key() {
        let song = SongPreviewInfo {
            id: 3,
            description: "Preview".to_string(),
            music_info: Key::new(Note::CNat, Mode::Minor),
        };

        let altered = song.alter_by_steps(-1).await.unwrap();

        assert_eq!(altered.id, 3);
        assert_eq!(altered.description, "Preview");
        assert_eq!(altered.music_info, Key::new(Note::FNat, Mode::Minor));
    }

    #[test]
    fn song_preview_info_display_includes_key() {
        let song = SongPreviewInfo {
            id: 2,
            description: "Preview Tune".to_string(),
            music_info: Key::new(Note::BFlat, Mode::Major),
        };

        assert_eq!(song.to_string(), "\nPreview Tune - Key:Bb");
    }

    #[tokio::test]
    async fn song_full_info_db_methods_return_error_when_pool_is_unavailable() {
        let pool = lazy_pool();

        assert!(SongFullInfo::get_all(&pool).await.is_err());
        assert!(SongFullInfo::get_songs_by_name(&pool, "blue").await.is_err());
        assert!(SongFullInfo::show_default(&pool, 1).await.is_err());
        assert!(SongFullInfo::alter_by_db(&pool, 1).await.is_err());
    }

    #[tokio::test]
    async fn song_preview_info_db_methods_return_error_when_pool_is_unavailable() {
        let pool = lazy_pool();
        let song = SongPreviewInfo {
            id: 1,
            description: "Preview".to_string(),
            music_info: Key::new(Note::CNat, Mode::Major),
        };

        assert!(SongPreviewInfo::get_all(&pool).await.is_err());
        assert!(
            SongPreviewInfo::get_songs_by_name(&pool, "blue")
                .await
                .is_err()
        );
        assert!(SongPreviewInfo::show_default(&pool, 1).await.is_err());
        assert!(song.alter_by_db(&pool).await.is_err());
    }

    #[test]
    fn song_full_info_from_raw_songs_filters_invalid_harmony() {
        let songs = SongFullInfo::from_raw_songs(vec![
            SongFullInfoRaw {
                song_id: 1,
                song_description: "Valid".to_string(),
                harmony: "A) |Cmaj7|".to_string(),
            },
            SongFullInfoRaw {
                song_id: 2,
                song_description: "Invalid".to_string(),
                harmony: "A) |H|".to_string(),
            },
        ]);

        assert_eq!(songs.len(), 1);
        assert_eq!(songs[0].id, 1);
        assert_eq!(songs[0].description, "Valid");
    }

    #[test]
    fn song_full_info_from_default_parts_formats_description_and_harmony() {
        let song = SongFullInfo::from_default_parts(
            5,
            Some(SongDescription {
                id: 5,
                song_name: "Name".to_string(),
                song_version: Some("Version".to_string()),
            }),
            Some(SongMusicInfo {
                song_id: 5,
                chords: "A) |Dm7 G7|".to_string(),
                key: "C".to_string(),
            }),
        )
        .unwrap();

        assert_eq!(song.id, 5);
        assert_eq!(song.description, "Name - Version");
        assert_eq!(song.music_info.parts[0].bars[0].chords.len(), 2);
    }

    #[test]
    fn song_full_info_from_default_parts_uses_fallbacks_and_reports_bad_harmony() {
        let fallback = SongFullInfo::from_default_parts(404, None, None).unwrap();
        assert_eq!(fallback.description, "Song with id 404 not found");
        assert!(fallback.music_info.parts.is_empty());

        let error = SongFullInfo::from_default_parts(
            1,
            None,
            Some(SongMusicInfo {
                song_id: 1,
                chords: "A) |H|".to_string(),
                key: "C".to_string(),
            }),
        )
        .unwrap_err();
        assert!(error.to_string().contains("Harmony can't be parsed"));
    }

    #[test]
    fn song_full_info_steps_default_and_alter_helper() {
        assert_eq!(SongFullInfo::steps_or_default(1, None), 0);

        let altered = SongFullInfo::from_altered_default(
            SongFullInfo {
                id: 1,
                description: "Tune".to_string(),
                music_info: harmony(),
            },
            1,
        );

        assert_eq!(altered.music_info.parts[0].bars[0].chords[0].to_string(), "G/B");
    }

    #[test]
    fn song_preview_info_from_raw_songs_filters_invalid_keys() {
        let songs = SongPreviewInfo::from_raw_songs(vec![
            SongPreviewRaw {
                song_id: 1,
                song_name: "Valid".to_string(),
                harmony_key: "Cm".to_string(),
            },
            SongPreviewRaw {
                song_id: 2,
                song_name: "Invalid".to_string(),
                harmony_key: "Hm".to_string(),
            },
        ]);

        assert_eq!(songs.len(), 1);
        assert_eq!(songs[0].id, 1);
        assert_eq!(songs[0].music_info, Key::new(Note::CNat, Mode::Minor));
    }

    #[test]
    fn song_preview_info_from_default_parts_uses_data_or_returns_parse_error() {
        let song = SongPreviewInfo::from_default_parts(
            3,
            Some(SongDescription {
                id: 3,
                song_name: "Preview".to_string(),
                song_version: None,
            }),
            Some(SongMusicInfo {
                song_id: 3,
                chords: "A) |C|".to_string(),
                key: "F".to_string(),
            }),
        )
        .unwrap();

        assert_eq!(song.description, "Preview");
        assert_eq!(song.music_info, Key::new(Note::FNat, Mode::Major));

        let error = SongPreviewInfo::from_default_parts(404, None, None).unwrap_err();
        assert!(error.to_string().contains("Key can't be parsed"));
    }

    #[test]
    fn song_preview_info_steps_default_and_alter_helper() {
        assert_eq!(SongPreviewInfo::steps_or_default(2, None), 0);

        let altered = SongPreviewInfo::from_altered_default(
            SongPreviewInfo {
                id: 2,
                description: "Preview".to_string(),
                music_info: Key::new(Note::CNat, Mode::Major),
            },
            2,
        );

        assert_eq!(altered.music_info, Key::new(Note::DNat, Mode::Major));
    }
}
