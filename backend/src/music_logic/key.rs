use super::Note;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    note: Note,
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Major,
    Minor,
}

impl Key {
    pub fn new(note: Note, mode: Mode) -> Self {
        Self { note, mode }
    }

    pub fn as_str(&self) -> String {
        match self.mode {
            Mode::Major => self.note.as_str().to_string(),
            Mode::Minor => format!("{}m", self.note.as_str()),
        }
    }

    pub fn from_db_string(s: &str) -> Result<Self, &str> {
        let cleaned = s.trim();

        if cleaned.ends_with('m') {
            let note_str = &cleaned[..cleaned.len() - 1];
            let note = Note::from_db_string(note_str)?;
            Ok(Key::new(note, Mode::Minor))
        } else {
            let note = Note::from_db_string(cleaned)?;
            Ok(Key::new(note, Mode::Major))
        }
    }

    //ALTERATION
    pub fn get_key_index(&self) -> usize {
        self.note.get_note_index()
    }

    pub fn next_key(&self) -> Self {
        Key::new(self.note.next_note(), self.mode)
    }

    pub fn alter_by_steps(&self, steps: i32) -> Self {
        Key::new(self.note.alter_by_steps(steps), self.mode)
    }
}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_key_with_correct_note_and_mode() {
        let key = Key::new(Note::FSharp, Mode::Minor);
        assert_eq!(key.note, Note::FSharp);
        assert_eq!(key.mode, Mode::Minor);

        let key = Key::new(Note::BFlat, Mode::Major);
        assert_eq!(key.note, Note::BFlat);
        assert_eq!(key.mode, Mode::Major);

        let key = Key::new(Note::AFlat, Mode::Minor);
        assert_eq!(key.note, Note::AFlat);
        assert_eq!(key.mode, Mode::Minor);
    }

    #[test]
    fn test_key_as_str_correct() {
        let k = Key::new(Note::CSharp, Mode::Major);
        assert_eq!(k.as_str(), "C#");

        let k = Key::new(Note::FNat, Mode::Minor);
        assert_eq!(k.as_str(), "Fm");

        let k = Key::new(Note::EFlat, Mode::Minor);
        assert_eq!(k.as_str(), "Ebm")
    }

    #[test]
    fn test_key_from_db_string_valid_inputs() {
        let k = Key::new(Note::CNat, Mode::Minor);
        assert_eq!(Key::from_db_string("Cm").unwrap(), k);

        let k = Key::new(Note::FSharp, Mode::Major);
        assert_eq!(Key::from_db_string("Gb").unwrap(), k);

        let k = Key::new(Note::AFlat, Mode::Minor);
        assert_eq!(Key::from_db_string("G#m").unwrap(), k);
    }

    #[test]
    fn test_key_from_db_string_invalid_inputs() {
        assert!(Key::from_db_string("Hm").is_err());
        assert!(Key::from_db_string("").is_err());
        assert!(Key::from_db_string("   ").is_err());
    }

    #[test]
    fn test_key_index_next_and_alter_preserve_mode() {
        let key = Key::new(Note::FNat, Mode::Minor);
        assert_eq!(key.get_key_index(), Note::FNat.get_note_index());
        assert_eq!(key.next_key(), Key::new(Note::CNat, Mode::Minor));
        assert_eq!(key.alter_by_steps(2), Key::new(Note::GNat, Mode::Minor));
        assert_eq!(key.alter_by_steps(-1), Key::new(Note::BFlat, Mode::Minor));
    }

    #[test]
    fn test_key_display_uses_as_str() {
        assert_eq!(Key::new(Note::BFlat, Mode::Major).to_string(), "Bb");
        assert_eq!(Key::new(Note::CSharp, Mode::Minor).to_string(), "C#m");
    }
}
