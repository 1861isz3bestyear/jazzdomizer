use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct SongDescription {
    pub id: i32,
    pub song_name: String,
    pub song_version: Option<String>,
}

impl SongDescription {
    pub async fn get_song_description_by_id(
        pool: &sqlx::PgPool,
        id: i32,
    ) -> Result<Option<SongDescription>, sqlx::Error> {
        let song = sqlx::query_as::<_, SongDescription>(
            "SELECT id, song_name, song_version
            FROM songs
            WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(song)
    }
}

#[derive(Debug, FromRow)]
pub struct SongMusicInfo {
    pub song_id: i32,
    pub chords: String,
    pub key: String,
}

impl SongMusicInfo {
    pub async fn get_song_music_info(
        pool: &sqlx::PgPool,
        search_id: i32,
    ) -> Result<Option<SongMusicInfo>, sqlx::Error> {
        let song = sqlx::query_as::<_, SongMusicInfo>(
            "SELECT song_id, chords, key
            FROM harmony
            WHERE song_id = $1",
        )
        .bind(search_id)
        .fetch_optional(pool)
        .await?;
        Ok(song)
    }
}

#[derive(Debug, FromRow)]
pub struct SongListInfo {
    pub song_id: i32,
    pub key_alteration: i32,
    pub complexity: i32,
}

impl SongListInfo {
    pub async fn get_song_list_info(
        pool: &sqlx::PgPool,
        search_id: i32,
    ) -> Result<Option<SongListInfo>, sqlx::Error> {
        let song = sqlx::query_as::<_, SongListInfo>(
            "SELECT song_id, complexity, key_alteration
            FROM list_data
            WHERE song_id = $1",
        )
        .bind(search_id)
        .fetch_optional(pool)
        .await?;
        Ok(song)
    }

    pub async fn drop_to_default(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            update list_data
            set
                global_playlist = false,
                local_playlist = false,
                song_for_study = false;
            "#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn mark_song_for_study(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            update list_data
            set song_for_study = true
            where id = (select max(song_id) from list_data);
            "#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn mark_songs_for_global_playlist(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            update list_data
            set global_playlist = true
            where song_for_study = false;
            "#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn unmark_songs_for_global_playlist(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            update list_data
            set global_playlist = false 
            where local_playlist = true;
            "#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn mark_songs_for_local_playlist(
        song_amount: i64,
        pool: &sqlx::PgPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            update list_data
            set local_playlist = true
            where song_id in (
                select song_id
                from list_data
                where global_playlist = true
                and local_playlist = false
                and song_for_study = false
                order by song_id
                limit $1
            );
            "#,
        )
        .bind(song_amount)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_song_amount_per_iteration(pool: &sqlx::PgPool) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            select count(*) from list_data
            where local_playlist = true;
            "#,
        )
        .fetch_one(pool)
        .await?;
        Ok(count)
    }

    pub async fn unmark_songs_for_local_playlist(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            update list_data
            set local_playlist = false 
            where local_playlist = true;
            "#,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[derive(Debug, FromRow)]
pub struct SongFullInfoRaw {
    pub song_id: i32,
    pub song_description: String,
    pub harmony: String,
}

impl SongFullInfoRaw {
    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let songs = sqlx::query_as::<_, SongFullInfoRaw>(
            "
            SELECT 
                s.id AS song_id,
                CONCAT(s.song_name, ' ', COALESCE(s.song_version, '')) AS song_description,
                h.key AS harmony
            FROM
                songs s
            INNER JOIN
                harmony h ON s.id = h.song_id
            ORDER BY
                s.song_name
            ",
        )
        .fetch_all(pool)
        .await?;
        Ok(songs)
    }

    pub async fn get_songs_by_part_name(
        pool: &sqlx::PgPool,
        search_term: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        if search_term.is_empty() {
            return Ok(Vec::new());
        }
        let pattern = format!("%{}%", search_term);
        let songs = sqlx::query_as::<_, SongFullInfoRaw>(
            "
            SELECT 
                s.id AS song_id, 
                CONCAT(s.song_name, ' ', COALESCE(s.song_version, '')) AS song_description,
                h.chords AS harmony
            FROM 
                songs s
            INNER JOIN 
                harmony h ON s.id = h.song_id
            WHERE 
                s.song_name ILIKE $1
            ORDER BY 
                s.song_name
            ",
        )
        .bind(&pattern)
        .fetch_all(pool)
        .await?;
        Ok(songs)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct SongPreviewRaw {
    pub song_id: i32,
    pub song_name: String,
    pub harmony_key: String,
}

impl SongPreviewRaw {
    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let songs = sqlx::query_as::<_, SongPreviewRaw>(
            "
            SELECT 
                s.id AS song_id,
                s.song_name,
                h.key AS harmony_key
            FROM
                songs s
            INNER JOIN
                harmony h ON s.id = h.song_id
            ORDER BY
                s.song_name
            ",
        )
        .fetch_all(pool)
        .await?;
        Ok(songs)
    }

    pub async fn get_songs_by_part_name(
        pool: &sqlx::PgPool,
        search_term: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        if search_term.is_empty() {
            return Ok(Vec::new());
        }
        let pattern = format!("%{}%", search_term);
        let songs = sqlx::query_as::<_, SongPreviewRaw>(
            "
            SELECT 
                s.id AS song_id, 
                s.song_name AS song_name,
                h.key AS harmony_key
            FROM 
                songs s
            INNER JOIN 
                harmony h ON s.id = h.song_id
            WHERE 
                s.song_name ILIKE $1
            ORDER BY 
                s.song_name
            ",
        )
        .bind(&pattern)
        .fetch_all(pool)
        .await?;
        Ok(songs)
    }

    pub async fn get_song_for_study(pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
        let song = sqlx::query_as::<_, SongPreviewRaw>(
            r#"
            select 
            s.id as song_id,
            s.song_name,
            h.key as harmony_key
            from list_data ld
            inner join songs s on ld.song_id = s.id
            inner join harmony h on ld.song_id = h.song_id
            where ld.song_for_study = true;
            "#,
        )
        .fetch_optional(pool)
        .await?;
        Ok(song)
    }

    pub async fn get_local_playlist_songs(pool: &sqlx::PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let songs = sqlx::query_as::<_, SongPreviewRaw>(
            r#"
            select 
                s.id as song_id,
                s.song_name,
                h.key as harmony_key
            from list_data ld
            inner join songs s on ld.song_id = s.id
            inner join harmony h on ld.song_id = h.song_id
            where ld.local_playlist = true
              and ld.song_for_study = false;
            "#,
        )
        .fetch_all(pool)
        .await?;
        Ok(songs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::PgPool;
    use std::time::Duration;

    #[cfg(not(coverage))]
    async fn arrange_song_data(pool: &PgPool) -> Result<(), sqlx::Error> {
        for statement in [
            r#"
            create table songs (
                id integer primary key,
                song_name text not null,
                song_version text
            );
            "#,
            r#"
            create table harmony (
                song_id integer primary key references songs(id),
                chords text not null,
                key text not null
            );
            "#,
            r#"
            create table list_data (
                id integer primary key,
                song_id integer not null references songs(id),
                key_alteration integer not null,
                complexity integer not null,
                global_playlist boolean not null default false,
                local_playlist boolean not null default false,
                song_for_study boolean not null default false
            );
            "#,
            r#"
            insert into songs (id, song_name, song_version) values
                (1, 'Blue Bossa', null),
                (2, 'Autumn Leaves', 'Take 1'),
                (3, 'Study Tune', null),
                (4, 'Local Only', null);
            "#,
            r#"
            insert into harmony (song_id, chords, key) values
                (1, 'Dm7 G7 Cmaj7', 'C'),
                (2, 'Am7 D7 Gmaj7', 'G'),
                (3, 'Fm7 Bb7 Ebmaj7', 'Eb'),
                (4, 'Em7 A7 Dmaj7', 'D');
            "#,
            r#"
            insert into list_data (
                id,
                song_id,
                key_alteration,
                complexity,
                global_playlist,
                local_playlist,
                song_for_study
            ) values
                (1, 1, 0, 3, true, false, false),
                (2, 2, -1, 4, true, false, false),
                (3, 3, 2, 5, false, false, true),
                (4, 4, 1, 2, false, true, false);
            "#,
        ] {
            sqlx::query(statement).execute(pool).await?;
        }

        Ok(())
    }

    #[cfg(not(coverage))]
    fn preview_ids(songs: &[SongPreviewRaw]) -> Vec<i32> {
        songs.iter().map(|song| song.song_id).collect()
    }

    fn lazy_pool() -> PgPool {
        PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(25))
            .connect_lazy("postgres://invalid:invalid@127.0.0.1:1/invalid?connect_timeout=1")
            .unwrap()
    }

    #[tokio::test]
    async fn read_queries_return_errors_when_pool_is_unavailable() {
        let pool = lazy_pool();

        assert!(
            SongDescription::get_song_description_by_id(&pool, 1)
                .await
                .is_err()
        );
        assert!(SongMusicInfo::get_song_music_info(&pool, 1).await.is_err());
        assert!(SongListInfo::get_song_list_info(&pool, 1).await.is_err());
        assert!(SongFullInfoRaw::get_all(&pool).await.is_err());
        assert!(
            SongFullInfoRaw::get_songs_by_part_name(&pool, "blue")
                .await
                .is_err()
        );
        assert!(SongPreviewRaw::get_all(&pool).await.is_err());
        assert!(
            SongPreviewRaw::get_songs_by_part_name(&pool, "blue")
                .await
                .is_err()
        );
        assert!(SongPreviewRaw::get_song_for_study(&pool).await.is_err());
        assert!(
            SongPreviewRaw::get_local_playlist_songs(&pool)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn empty_search_terms_short_circuit_without_database() {
        let pool = lazy_pool();

        assert!(
            SongFullInfoRaw::get_songs_by_part_name(&pool, "")
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            SongPreviewRaw::get_songs_by_part_name(&pool, "")
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn write_queries_return_errors_when_pool_is_unavailable() {
        let pool = lazy_pool();

        assert!(SongListInfo::drop_to_default(&pool).await.is_err());
        assert!(SongListInfo::mark_song_for_study(&pool).await.is_err());
        assert!(
            SongListInfo::mark_songs_for_global_playlist(&pool)
                .await
                .is_err()
        );
        assert!(
            SongListInfo::unmark_songs_for_global_playlist(&pool)
                .await
                .is_err()
        );
        assert!(
            SongListInfo::mark_songs_for_local_playlist(4, &pool)
                .await
                .is_err()
        );
        assert!(
            SongListInfo::get_song_amount_per_iteration(&pool)
                .await
                .is_err()
        );
        assert!(
            SongListInfo::unmark_songs_for_local_playlist(&pool)
                .await
                .is_err()
        );
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn gets_song_description_by_id(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        let song = SongDescription::get_song_description_by_id(&pool, 2)
            .await?
            .expect("seeded song should exist");
        assert_eq!(song.id, 2);
        assert_eq!(song.song_name, "Autumn Leaves");
        assert_eq!(song.song_version.as_deref(), Some("Take 1"));
        assert!(
            SongDescription::get_song_description_by_id(&pool, 404)
                .await?
                .is_none()
        );
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn gets_song_music_info(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        let song = SongMusicInfo::get_song_music_info(&pool, 1)
            .await?
            .expect("seeded harmony should exist");
        assert_eq!(song.song_id, 1);
        assert_eq!(song.chords, "Dm7 G7 Cmaj7");
        assert_eq!(song.key, "C");
        assert!(
            SongMusicInfo::get_song_music_info(&pool, 404)
                .await?
                .is_none()
        );
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn gets_song_list_info(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        let song = SongListInfo::get_song_list_info(&pool, 2)
            .await?
            .expect("seeded list data should exist");
        assert_eq!(song.song_id, 2);
        assert_eq!(song.key_alteration, -1);
        assert_eq!(song.complexity, 4);
        assert!(
            SongListInfo::get_song_list_info(&pool, 404)
                .await?
                .is_none()
        );
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn resets_playlist_flags_to_default(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        SongListInfo::drop_to_default(&pool).await?;
        assert_eq!(SongListInfo::get_song_amount_per_iteration(&pool).await?, 0);
        assert!(SongPreviewRaw::get_song_for_study(&pool).await?.is_none());
        assert!(
            SongPreviewRaw::get_local_playlist_songs(&pool)
                .await?
                .is_empty()
        );
        SongListInfo::mark_songs_for_local_playlist(4, &pool).await?;
        assert!(
            SongPreviewRaw::get_local_playlist_songs(&pool)
                .await?
                .is_empty()
        );
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn marks_latest_song_for_study(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        SongListInfo::drop_to_default(&pool).await?;
        SongListInfo::mark_song_for_study(&pool).await?;
        let song = SongPreviewRaw::get_song_for_study(&pool)
            .await?
            .expect("one song should be marked for study");
        assert_eq!(song.song_id, 4);
        assert_eq!(song.song_name, "Local Only");
        assert_eq!(song.harmony_key, "D");
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn builds_local_playlist_from_global_playlist(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        SongListInfo::drop_to_default(&pool).await?;
        SongListInfo::mark_song_for_study(&pool).await?;
        SongListInfo::mark_songs_for_global_playlist(&pool).await?;
        SongListInfo::mark_songs_for_local_playlist(2, &pool).await?;
        let songs = SongPreviewRaw::get_local_playlist_songs(&pool).await?;
        assert_eq!(preview_ids(&songs), vec![1, 2]);
        assert_eq!(SongListInfo::get_song_amount_per_iteration(&pool).await?, 2);
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn unmarks_playlist_flags(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        SongListInfo::drop_to_default(&pool).await?;
        SongListInfo::mark_song_for_study(&pool).await?;
        SongListInfo::mark_songs_for_global_playlist(&pool).await?;
        SongListInfo::mark_songs_for_local_playlist(2, &pool).await?;
        SongListInfo::unmark_songs_for_global_playlist(&pool).await?;
        SongListInfo::unmark_songs_for_local_playlist(&pool).await?;
        SongListInfo::mark_songs_for_local_playlist(4, &pool).await?;
        let songs = SongPreviewRaw::get_local_playlist_songs(&pool).await?;
        assert_eq!(preview_ids(&songs), vec![3]);
        assert_eq!(SongListInfo::get_song_amount_per_iteration(&pool).await?, 1);
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn gets_all_full_song_info_in_name_order(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        let songs = SongFullInfoRaw::get_all(&pool).await?;
        assert_eq!(
            songs
                .iter()
                .map(|song| (
                    song.song_id,
                    song.song_description.as_str(),
                    song.harmony.as_str()
                ))
                .collect::<Vec<_>>(),
            vec![
                (2, "Autumn Leaves Take 1", "G"),
                (1, "Blue Bossa ", "C"),
                (4, "Local Only ", "D"),
                (3, "Study Tune ", "Eb"),
            ]
        );
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn searches_full_song_info_by_name_part(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        assert!(
            SongFullInfoRaw::get_songs_by_part_name(&pool, "")
                .await?
                .is_empty()
        );
        let songs = SongFullInfoRaw::get_songs_by_part_name(&pool, "blue").await?;
        assert_eq!(songs.len(), 1);
        assert_eq!(songs[0].song_id, 1);
        assert_eq!(songs[0].song_description, "Blue Bossa ");
        assert_eq!(songs[0].harmony, "Dm7 G7 Cmaj7");
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn searches_preview_song_info_by_name_part(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        assert!(
            SongPreviewRaw::get_songs_by_part_name(&pool, "")
                .await?
                .is_empty()
        );
        let songs = SongPreviewRaw::get_songs_by_part_name(&pool, "blue").await?;
        assert_eq!(songs.len(), 1);
        assert_eq!(songs[0].song_id, 1);
        assert_eq!(songs[0].song_name, "Blue Bossa");
        assert_eq!(songs[0].harmony_key, "C");
        Ok(())
    }

    #[sqlx::test(migrations = false)]
    #[ignore = "requires a configured Postgres test database"]
    #[cfg(not(coverage))]
    async fn gets_preview_song_info(pool: PgPool) -> Result<(), sqlx::Error> {
        arrange_song_data(&pool).await?;
        let all_songs = SongPreviewRaw::get_all(&pool).await?;
        let study_song = SongPreviewRaw::get_song_for_study(&pool)
            .await?
            .expect("seeded study song should exist");
        let local_songs = SongPreviewRaw::get_local_playlist_songs(&pool).await?;
        assert_eq!(preview_ids(&all_songs), vec![2, 1, 4, 3]);
        assert_eq!(study_song.song_id, 3);
        assert_eq!(study_song.song_name, "Study Tune");
        assert_eq!(study_song.harmony_key, "Eb");
        assert_eq!(preview_ids(&local_songs), vec![4]);
        Ok(())
    }
}
