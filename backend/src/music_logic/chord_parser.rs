use super::Note;
use crate::music_logic::ChordForReading;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub enum ParseError {
    EmptyString,
    InvalidFormat(String),
    InvalidNote(String),
    RegexError(String),
    NoNoteFound,
}

static NOTE_REGEX: Lazy<Result<Regex, ParseError>> =
    Lazy::new(|| match Regex::new(r"^([A-G][#b]?)") {
        Ok(re) => Ok(re),
        Err(e) => Err(ParseError::RegexError(e.to_string())),
    });

fn extract_note(s: &str) -> Option<Note> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    let regex = NOTE_REGEX.as_ref().ok()?;
    let caps = regex.captures(trimmed)?;
    let note_str = caps.get(1)?.as_str();
    Note::from_db_string(note_str).ok()
}

pub fn parse_root(chord_str: &str) -> Result<Note, ParseError> {
    let main_part = chord_str.split('/').next().unwrap_or(chord_str);
    if main_part.trim().is_empty() {
        return Err(ParseError::EmptyString);
    }
    extract_note(main_part).ok_or_else(|| ParseError::NoNoteFound)
}

pub fn parse_bass(chord_str: &str) -> Result<Option<Note>, ParseError> {
    let parts: Vec<&str> = chord_str.split('/').collect();
    if parts.len() > 1 {
        let bass_part = parts[1].trim();
        if bass_part.is_empty() {
            Ok(None)
        } else {
            match extract_note(bass_part) {
                Some(note) => Ok(Some(note)),
                None => Err(ParseError::InvalidFormat(format!(
                    "Неверный формат басовой ноты {}",
                    bass_part
                ))),
            }
        }
    } else {
        Ok(None)
    }
}

pub fn parse_color(chord_str: &str) -> Result<Option<String>, ParseError> {
    let chord_str = chord_str.trim();
    if chord_str.is_empty() {
        return Err(ParseError::EmptyString);
    }
    let main_part = chord_str.split('/').next().unwrap_or(chord_str);
    let regex = NOTE_REGEX
        .as_ref()
        .map_err(|e| ParseError::RegexError(format!("{:?}", e)))?;
    let caps = regex.captures(main_part).ok_or(ParseError::NoNoteFound)?;
    let root_str = caps.get(1).ok_or(ParseError::NoNoteFound)?.as_str();
    if main_part.len() <= root_str.len() {
        return Ok(None);
    }
    let color = &main_part[root_str.len()..];
    Ok(if color.is_empty() {
        None
    } else {
        Some(color.trim().to_string())
    })
}

pub fn parse_chord(chord_str: &str) -> Result<ChordForReading, ParseError> {
    let chord_str = chord_str.trim();
    if chord_str.is_empty() {
        return Err(ParseError::EmptyString);
    }
    let root = parse_root(chord_str)?;
    let color = parse_color(chord_str)?;
    let bass = parse_bass(chord_str)?;
    Ok(ChordForReading { root, color, bass })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_parser_valid_notes() {
        let test_cases = vec![
            ("C", Note::CNat),
            ("C#", Note::CSharp),
            ("Db", Note::CSharp),
            ("D", Note::DNat),
            ("D#", Note::EFlat),
            ("Eb", Note::EFlat),
            ("E", Note::ENat),
            ("F", Note::FNat),
            ("F#", Note::FSharp),
            ("Gb", Note::FSharp),
            ("G", Note::GNat),
            ("G#", Note::AFlat),
            ("Ab", Note::AFlat),
            ("A", Note::ANat),
            ("A#", Note::BFlat),
            ("Bb", Note::BFlat),
            ("B", Note::BNat),
        ];
        for (input, expected) in test_cases {
            let result = parse_root(input);
            assert_eq!(result.unwrap(), expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_parse_root_with_whitespace() {
        assert_eq!(parse_root("  C  ").unwrap(), Note::CNat);
        assert_eq!(parse_root("\tC#\n").unwrap(), Note::CSharp);
    }

    #[test]
    fn test_parse_root_with_color() {
        // Должен игнорировать цвет и извлекать только ноту
        assert_eq!(parse_root("Cmaj7").unwrap(), Note::CNat);
        assert_eq!(parse_root("Dm7").unwrap(), Note::DNat);
        assert_eq!(parse_root("G#m7b5").unwrap(), Note::AFlat);
    }

    #[test]
    fn test_parse_root_with_bass() {
        // Должен брать часть до слеша
        assert_eq!(parse_root("C/G").unwrap(), Note::CNat);
        assert_eq!(parse_root("Am/C").unwrap(), Note::ANat);
    }

    #[test]
    fn test_parse_root_empty_string() {
        let result = parse_root("");
        assert!(matches!(result, Err(ParseError::EmptyString)));
    }

    #[test]
    fn test_parse_root_whitespace_only() {
        let result = parse_root("   ");
        assert!(matches!(result, Err(ParseError::EmptyString)));
    }

    #[test]
    fn test_parse_root_invalid_note() {
        let test_cases = vec!["H", "X", "H#", "p##", "wbb", "123"];
        for input in test_cases {
            let result = parse_root(input);
            assert!(matches!(result, Err(ParseError::NoNoteFound)));
        }
    }

    #[test]
    fn test_parse_bass_no_bass() {
        let test_cases = vec!["C", "Cmaj7", "Dm7", "   G   "];
        for input in test_cases {
            let result = parse_bass(input);
            assert_eq!(result.unwrap(), None, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_parse_bass_with_bass() {
        let test_cases = vec![
            ("C/G", Note::GNat),
            ("Am/C", Note::CNat),
            ("F#m7/B", Note::BNat),
            ("Bb/D", Note::DNat),
        ];
        for (input, expected) in test_cases {
            let result = parse_bass(input);
            assert_eq!(
                result.unwrap(),
                Some(expected),
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_parse_bass_with_whitespace() {
        assert_eq!(parse_bass("C/  G  ").unwrap(), Some(Note::GNat));
        assert_eq!(parse_bass("C/\tG#\n").unwrap(), Some(Note::AFlat));
    }

    #[test]
    fn test_parse_bass_empty_bass() {
        // Случаи вроде "C/" должны возвращать None
        let result = parse_bass("C/");
        assert_eq!(result.unwrap(), None);
        let result = parse_bass("C/   ");
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_bass_invalid_bass() {
        let test_cases = vec!["C/H", "D/X", "G/123"];
        for input in test_cases {
            let result = parse_bass(input);
            assert!(
                matches!(result, Err(ParseError::InvalidFormat(_))),
                "Expected InvalidFormat for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_parse_bass_multiple_slashes() {
        // Должен обрабатывать только первый слеш
        let result = parse_bass("C/G/B");
        assert_eq!(result.unwrap(), Some(Note::GNat));
    }
}
