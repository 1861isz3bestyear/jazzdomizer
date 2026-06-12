use crate::db::{SongListInfo, SongPreviewRaw};

#[derive(Debug, Clone)]
pub struct GlobalSongList {
    songs_per_iteration: i64,
    pool: sqlx::PgPool,
}

impl GlobalSongList {
    pub async fn new(songs_per_iteration: i64, pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
        SongListInfo::drop_to_default(pool).await?;
        SongListInfo::mark_song_for_study(pool).await?;
        SongListInfo::mark_songs_for_global_playlist(pool).await?;
        SongListInfo::mark_songs_for_local_playlist(songs_per_iteration, pool).await?;
        Ok(Self {
            songs_per_iteration,
            pool: pool.clone(),
        })
    }

    pub async fn next_local_playlist(pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
        let n: i64 = SongListInfo::get_song_amount_per_iteration(pool)
            .await
            .expect("Can't find in DB");
        SongListInfo::unmark_songs_for_global_playlist(pool).await?;
        SongListInfo::unmark_songs_for_local_playlist(pool).await?;
        SongListInfo::mark_songs_for_local_playlist(n, pool).await?;
        Ok(Self {
            songs_per_iteration: n,
            pool: pool.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LocalSongList {
    pub preview_songs: Vec<SongPreviewRaw>,
    pool: sqlx::PgPool,
}

impl LocalSongList {
    pub async fn check_practice_records(pool: &sqlx::PgPool) -> Result<bool, sqlx::Error> {
        let study_songs = SongPreviewRaw::get_song_for_study(pool).await?;
        let local_playlist_songs = SongPreviewRaw::get_local_playlist_songs(pool).await?;
        Ok(study_songs.is_some() && !local_playlist_songs.is_empty())
    }

    pub async fn continue_session(pool: &sqlx::PgPool) -> Result<Self, sqlx::Error> {
        let mut list_songs = SongPreviewRaw::get_local_playlist_songs(pool).await?;
        let study_song = SongPreviewRaw::get_song_for_study(pool).await?;
        let study_song = match study_song {
            Some(song) => song,
            None => {
                return Err(sqlx::Error::RowNotFound);
            }
        };
        let middle = list_songs.len() / 2;
        list_songs.insert(middle, study_song);
        Ok(Self {
            preview_songs: list_songs,
            pool: pool.clone(),
        })
    }
}
