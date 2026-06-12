use super::Note;
use super::parse_chord;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChordForReading {
    pub root: Note,
    pub color: Option<String>,
    pub bass: Option<Note>,
}

impl ChordForReading {
    pub fn new(root: Note, color: Option<&str>, bass: Option<Note>) -> Self {
        ChordForReading {
            root,
            color: color.map(|s| s.to_string()),
            bass,
        }
    }

    pub fn from_db_string(s: &str) -> Result<Self, String> {
        let cleaned = s.trim();
        parse_chord(cleaned).map_err(|e| format!("Ошибка парсинга аккорда '{}': {:?}", cleaned, e))
    }

    pub fn alter_by_steps(&self, steps: i32) -> Self {
        let altered_root = self.root.alter_by_steps(steps);
        let altered_bass = self.bass.map(|bass_note| bass_note.alter_by_steps(steps));

        ChordForReading {
            root: altered_root,
            color: self.color.clone(),
            bass: altered_bass,
        }
    }
}

impl fmt::Display for ChordForReading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = self.root.to_string();
        if let Some(color) = &self.color {
            result.push_str(color);
        }
        if let Some(bass_note) = &self.bass {
            result.push('/');
            result.push_str(&bass_note.to_string());
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChordForWriting {
    pub root: Note,
    pub quality: Quality,           //maj, min, dom
    pub extensions: Vec<Extension>, // #9, b9, etc
    pub bass: Option<Note>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quality {
    Major,
    Minor,
    Dom,
    HalfDim,
    Dim,
    Aug,
    Sus2,
    Sus4,
}

impl Quality {
    pub fn to_str(&self) -> &str {
        ["", "m", "7", "m7b5", "dim", "+", "sus2", "sus4"][*self as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Extension {
    None,
    Seven,
    FlatNinth,
    Ninth,
    SharpNinth,
    SharpEleven,
    FlatThirteen,
}

impl Extension {
    pub fn to_str(&self) -> &str {
        ["", "7", "b9", "9", "#9", "#11", "b13"][*self as usize]
    }
}

impl fmt::Display for ChordForWriting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = self.root.to_string();
        result.push_str(self.quality.to_str());
        for ext in &self.extensions {
            result.push_str(ext.to_str());
        }
        if let Some(bass_note) = &self.bass {
            result.push_str("/");
            result.push_str(&bass_note.to_string());
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_test_chord(root: Note, color: Option<&str>, bass: Option<Note>) -> ChordForReading {
        ChordForReading {
            root,
            color: color.map(|s| s.to_string()),
            bass,
        }
    }
    fn test_chord_reading_from_db_string() {
        let test_cases = vec![
            ("C", Note::CNat, None, None),
            ("Dbmaj7", Note::CSharp, Some("maj7"), None),
            ("F#m7b5", Note::FSharp, Some("m7b5"), None),
            ("Bb#11b13/C", Note::BFlat, Some("#11b13"), Some(Note::CNat)),
            ("Ebm11/Db", Note::EFlat, Some("m11"), Some(Note::CSharp)),
        ];
        for (input, expected_root, expected_color, expected_bass) in test_cases {
            let result = ChordForReading::from_db_string(input);
            assert!(result.is_ok(), "Failed for input: {}", input);
            let chord = result.unwrap();
            assert_eq!(chord.root, expected_root, "Root is mismatch for {}", input);
            assert_eq!(
                chord.color,
                expected_color.map(String::from),
                "Color is mismatch for {}",
                input
            );
            assert_eq!(chord.bass, expected_bass, "Bass mismatch for {}", input);
        }
    }

    #[test]
    fn test_chord_reading_from_db_string_whitespaces() {
        let test_cases = vec![
            (" G ", Note::GNat, None, None),
            ("\tBbmaj\n", Note::BFlat, Some("maj"), None),
            (" F#m7 / G ", Note::FSharp, Some("m7"), Some(Note::GNat)),
        ];
        for (input, expected_root, expected_color, expected_bass) in test_cases {
            let result = ChordForReading::from_db_string(input);
            assert!(result.is_ok(), "Failed for input: {}", input);
            let chord = result.unwrap();
            assert_eq!(chord.root, expected_root, "Root is mismatch for {}", input);
            assert_eq!(
                chord.color,
                expected_color.map(String::from),
                "Color is mismatch for {}",
                input
            );
            assert_eq!(chord.bass, expected_bass, "Bass mismatch for {}", input);
        }
    }

    #[test]
    fn test_alter_by_steps() {
        let test_cases = vec![
            (
                create_test_chord(Note::CNat, None, None),
                2,
                create_test_chord(Note::DNat, None, None),
            ),
            (
                create_test_chord(Note::CNat, Some("maj7"), None),
                -3,
                create_test_chord(Note::EFlat, Some("maj7"), None),
            ),
            (
                create_test_chord(Note::CNat, Some("maj7"), None),
                -2,
                create_test_chord(Note::BFlat, Some("maj7"), None),
            ),
            (
                create_test_chord(Note::CNat, Some("m7"), Some(Note::GNat)),
                5,
                create_test_chord(Note::BNat, Some("m7"), Some(Note::FSharp)),
            ),
            (
                create_test_chord(Note::BNat, None, None),
                1,
                create_test_chord(Note::FSharp, None, None),
            ),
            (
                create_test_chord(Note::CSharp, Some("7"), None),
                3,
                create_test_chord(Note::BFlat, Some("7"), None),
            ),
            (
                create_test_chord(Note::FSharp, Some("m7b5"), Some(Note::CSharp)),
                -3,
                create_test_chord(Note::ANat, Some("m7b5"), Some(Note::ENat)),
            ),
        ];

        for (chord, steps, expected) in test_cases {
            let result = chord.alter_by_steps(steps);
            assert_eq!(
                result, expected,
                "Failed for chord: {} with steps: {}",
                chord, steps
            );
        }
    }
}
