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
        let db_songs = SongFullInfoRaw::get_all(pool).await?;
        let songs = db_songs
            .into_iter()
            .filter_map(|raw| {
                let music_info = SongHarmony::from_db_string(&raw.harmony).ok()?;
                Some(SongFullInfo {
                    id: raw.song_id,
                    description: raw.song_description,
                    music_info,
                })
            })
            .collect::<Vec<_>>();
        Ok(songs)
    }

    pub async fn get_songs_by_name(
        pool: &PgPool,
        search_term: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let db_songs = SongFullInfoRaw::get_songs_by_part_name(pool, search_term).await?;
        let songs = db_songs
            .into_iter()
            .filter_map(|raw| {
                let music_info = SongHarmony::from_db_string(&raw.harmony).ok()?;
                Some(SongFullInfo {
                    id: raw.song_id,
                    description: raw.song_description,
                    music_info,
                })
            })
            .collect::<Vec<_>>();
        Ok(songs)
    }

    pub async fn show_default(pool: &PgPool, song_id: i32) -> Result<Self, sqlx::Error> {
        let db_description = SongDescription::get_song_description_by_id(pool, song_id).await?;
        let description = db_description
            .as_ref()
            .map(|desc| match &desc.song_version {
                Some(version) => format!("{} - {}", desc.song_name, version),
                None => desc.song_name.clone(),
            })
            .unwrap_or_else(|| format!("Song with id {} not found", song_id));
        let db_music_info = SongMusicInfo::get_song_music_info(pool, song_id).await?;
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

    pub async fn alter_by_db(pool: &PgPool, song_id: i32) -> Result<Self, sqlx::Error> {
        let db_description = SongListInfo::get_song_list_info(pool, song_id).await?;
        let steps = db_description
            .as_ref()
            .map(|info| info.key_alteration.clone())
            .unwrap_or_else(|| {
                eprintln!("No alteration data for song with id {}", song_id);
                0
            });
        let def = SongFullInfo::show_default(pool, song_id).await?;
        let altered = def.music_info.clone().alter_by_steps(steps);
        Ok(SongFullInfo {
            id: def.id,
            description: def.description,
            music_info: altered,
        })
    }

    pub async fn alter_by_steps(&self, steps: i32) -> Result<Self, sqlx::Error> {
        let alt_harmony = self.music_info.clone().alter_by_steps(steps);
        Ok(SongFullInfo {
            id: self.id,
            description: self.description.clone(),
            music_info: alt_harmony,
        })
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
        let db_songs = SongPreviewRaw::get_all(pool).await?;
        let songs = db_songs
            .into_iter()
            .filter_map(|raw| {
                let music_info = Key::from_db_string(&raw.harmony_key).ok()?;
                Some(SongPreviewInfo {
                    id: raw.song_id,
                    description: raw.song_name,
                    music_info,
                })
            })
            .collect::<Vec<_>>();
        Ok(songs)
    }

    pub async fn get_songs_by_name(
        pool: &PgPool,
        search_term: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let db_songs = SongPreviewRaw::get_songs_by_part_name(pool, search_term).await?;
        let songs = db_songs
            .into_iter()
            .filter_map(|raw| {
                let music_info = Key::from_db_string(&raw.harmony_key).ok()?;
                Some(SongPreviewInfo {
                    id: raw.song_id,
                    description: raw.song_name,
                    music_info,
                })
            })
            .collect::<Vec<_>>();
        Ok(songs)
    }

    pub async fn show_default(pool: &PgPool, song_id: i32) -> Result<Self, sqlx::Error> {
        let db_description = SongDescription::get_song_description_by_id(pool, song_id).await?;
        let description = match db_description {
            Some(desc) => desc.song_name,
            None => format!("Song with id {} not found", song_id),
        };
        let db_music_info = SongMusicInfo::get_song_music_info(pool, song_id).await?;
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

    pub async fn alter_by_db(&self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let db_description = SongListInfo::get_song_list_info(pool, self.id).await?;
        let steps = db_description
            .as_ref()
            .map(|info| info.key_alteration.clone())
            .unwrap_or_else(|| {
                eprintln!("No alteration data for song with id {}", self.id);
                0
            });
        let altered_harmony = self.music_info.clone().alter_by_steps(steps);
        Ok(SongPreviewInfo {
            id: self.id,
            description: self.description.clone(),
            music_info: altered_harmony,
        })
    }

    pub async fn alter_by_steps(&self, steps: i32) -> Result<Self, sqlx::Error> {
        let alt_key = self.music_info.clone().alter_by_steps(steps);
        Ok(SongPreviewInfo {
            id: self.id,
            description: self.description.clone(),
            music_info: alt_key,
        })
    }
}

impl fmt::Display for SongPreviewInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{} - Key:{}", self.description, self.music_info)
    }
}
