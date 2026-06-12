use super::Key;
use super::SongHarmony;
use crate::db::SongMusicInfo;

#[derive(Debug, Clone)]
pub struct Song {
    pub id: i32,
    pub key: Key,
    pub harmony: SongHarmony,
}

impl Song {
    pub fn from_db(song_db: SongMusicInfo) -> Result<Self, String> {
        let key = Key::from_db_string(&song_db.key)?;
        let harmony = SongHarmony::from_db_string(&song_db.chords)?;

        Ok(Self {
            id: song_db.song_id,
            key: key,
            harmony: harmony,
        })
    }

    pub fn alter_by_steps(self, steps: i32) -> Self {
        let alt_key = self.key.alter_by_steps(steps);
        let alt_harmony = self.harmony.alter_by_steps(steps);

        Self {
            id: self.id.clone(),
            key: alt_key,
            harmony: alt_harmony,
        }
    }
}
