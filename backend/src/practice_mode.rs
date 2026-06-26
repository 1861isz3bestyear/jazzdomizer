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

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::FutureExt;
    use sqlx::postgres::PgPoolOptions;
    use std::panic::AssertUnwindSafe;
    use std::time::Duration;

    fn lazy_pool() -> sqlx::PgPool {
        PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(25))
            .connect_lazy("postgres://invalid:invalid@127.0.0.1:1/invalid?connect_timeout=1")
            .unwrap()
    }

    #[tokio::test]
    async fn global_song_list_new_returns_db_error_when_pool_is_unavailable() {
        let pool = lazy_pool();
        let result = GlobalSongList::new(4, &pool).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn global_song_list_next_local_playlist_panics_when_song_count_cannot_load() {
        let pool = lazy_pool();
        let result = AssertUnwindSafe(GlobalSongList::next_local_playlist(&pool))
            .catch_unwind()
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn local_song_list_check_practice_records_returns_db_error_when_pool_is_unavailable() {
        let pool = lazy_pool();
        let result = LocalSongList::check_practice_records(&pool).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn local_song_list_continue_session_returns_db_error_when_pool_is_unavailable() {
        let pool = lazy_pool();
        let result = LocalSongList::continue_session(&pool).await;
        assert!(result.is_err());
    }
}
