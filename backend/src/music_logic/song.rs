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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::SongMusicInfo;
    use crate::music_logic::{Mode, Note};

    #[test]
    fn from_db_parses_key_and_harmony() {
        let song = Song::from_db(SongMusicInfo {
            song_id: 7,
            chords: "A) |Cmaj7|Dm7 G7|".to_string(),
            key: "C".to_string(),
        })
        .unwrap();

        assert_eq!(song.id, 7);
        assert_eq!(song.key, Key::new(Note::CNat, Mode::Major));
        assert_eq!(song.harmony.parts.len(), 1);
        assert_eq!(song.harmony.parts[0].bars.len(), 2);
    }

    #[test]
    fn from_db_returns_error_for_invalid_key_or_harmony() {
        let invalid_key = Song::from_db(SongMusicInfo {
            song_id: 1,
            chords: "A) |C|".to_string(),
            key: "H".to_string(),
        });
        assert!(invalid_key.is_err());

        let invalid_harmony = Song::from_db(SongMusicInfo {
            song_id: 1,
            chords: "A) |H|".to_string(),
            key: "C".to_string(),
        });
        assert!(invalid_harmony.is_err());
    }

    #[test]
    fn alter_by_steps_transposes_key_and_harmony() {
        let song = Song::from_db(SongMusicInfo {
            song_id: 4,
            chords: "A) |C/E|G7|".to_string(),
            key: "C".to_string(),
        })
        .unwrap();

        let altered = song.alter_by_steps(1);
        assert_eq!(altered.id, 4);
        assert_eq!(altered.key, Key::new(Note::GNat, Mode::Major));
        assert_eq!(altered.harmony.parts[0].bars[0].chords[0].to_string(), "G/B");
        assert_eq!(altered.harmony.parts[0].bars[1].chords[0].to_string(), "D7");
    }
}
