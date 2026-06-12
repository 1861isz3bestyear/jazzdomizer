use super::ChordForReading;

#[derive(Debug, Clone)]
pub struct SongHarmony {
    pub parts: Vec<SongPart>,
}

impl SongHarmony {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn from_db_string(text: &str) -> Result<Self, String> {
        let mut parts = Vec::new();
        let lines: Vec<&str> = text
            .trim()
            .split("\n")
            .filter(|s| !s.trim().is_empty())
            .collect();
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            if line.contains(")") {
                let mut part_text = line.to_string();
                i += 1;
                while i < lines.len() && !lines[i].contains(")") {
                    part_text.push_str("\n");
                    part_text.push_str(lines[i]);
                    i += 1;
                }
                match SongPart::from_text(&part_text) {
                    Ok(part) => parts.push(part),
                    Err(e) => return Err(format!("Ошибка парсинга части '{}': {}", line, e)),
                }
            } else {
                i += 1;
            }
        }
        Ok(Self { parts })
    }

    pub fn alter_by_steps(self, steps: i32) -> Self {
        let alt = self
            .parts
            .iter()
            .map(|part| part.alter_song_part(steps))
            .collect();
        SongHarmony { parts: alt }
    }
}

#[derive(Debug, Clone)]
pub struct SongPart {
    pub name: String,
    pub bars: Vec<Bar>,
}

impl SongPart {
    pub fn new(name: String) -> Self {
        Self {
            name,
            bars: Vec::new(),
        }
    }

    pub fn from_text(text: &str) -> Result<Self, String> {
        let (name, bars) = text
            .split_once(')')
            .ok_or_else(|| format!("Нет разделителя ')' в строке {}", text))?;
        let name = name.trim();
        let bars_text = bars.trim();
        if name.is_empty() {
            return Err("Имя части не может быть пустым".to_string());
        }
        let bars =
            Self::parse_bars(bars_text).map_err(|e| format!("Ошибка парсинга тактов {}", e))?;
        Ok(Self {
            name: name.to_string(),
            bars: bars,
        })
    }

    pub fn parse_bars(bars_text: &str) -> Result<Vec<Bar>, String> {
        let lines: Vec<&str> = bars_text
            .trim()
            .split("\n")
            .filter(|s| !s.trim().is_empty())
            .collect();
        let mut all_bars = Vec::new();
        let mut bar_counter = 1;
        for line in lines {
            let bar_texts_in_line: Vec<&str> =
                line.split("|").filter(|s| !s.trim().is_empty()).collect();
            for bar_text in bar_texts_in_line {
                match Bar::from_chord_text(bar_text) {
                    Ok(bar) => all_bars.push(bar),
                    Err(e) => {
                        return Err(format!(
                            "Ошибка в такте #{} ('{}'): {}",
                            bar_counter, bar_text, e
                        ));
                    }
                }
                bar_counter += 1;
            }
        }
        Ok(all_bars)
    }

    pub fn alter_song_part(&self, steps: i32) -> Self {
        let altered_bars: Vec<Bar> = self
            .bars
            .iter()
            .map(|bar| bar.alter_chords(steps))
            .collect();
        SongPart {
            name: self.name.clone(),
            bars: altered_bars,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub chords: Vec<ChordForReading>,
}

impl Bar {
    pub fn from_chord_text(text: &str) -> Result<Self, String> {
        let chords = Self::parse_chords(text)?;
        Ok(Bar { chords })
    }

    fn parse_chords(text: &str) -> Result<Vec<ChordForReading>, String> {
        let chords: Vec<ChordForReading> = text
            .trim()
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|chord_str| {
                ChordForReading::from_db_string(chord_str)
                    .map_err(|e| format!("Ошибка в аккорде '{}': {}", chord_str, e))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(chords)
    }

    pub fn alter_chords(&self, steps: i32) -> Self {
        let altered_chords: Vec<ChordForReading> = self
            .chords
            .iter()
            .map(|chord| chord.alter_by_steps(steps))
            .collect();
        Bar {
            chords: altered_chords,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music_logic::Note;

    #[test]
    fn test_harmony_new_song() {
        let hrmn = SongHarmony::new();
        assert!(hrmn.parts.is_empty());
        assert_eq!(hrmn.parts.len(), 0);
    }

    #[test]
    fn test_harmony_single_part() {
        let txt = "verse) |C G| Am |";
        let hrmn = SongHarmony::from_db_string(txt).unwrap();
        assert_eq!(hrmn.parts.len(), 1);
        assert_eq!(hrmn.parts[0].name, "verse");
        assert_eq!(hrmn.parts[0].bars.len(), 2);
        assert_eq!(hrmn.parts[0].bars[0].chords[0].to_string(), "C");
        assert_eq!(hrmn.parts[0].bars[0].chords[1].to_string(), "G");
        assert_eq!(hrmn.parts[0].bars[1].chords[0].to_string(), "Am");
    }

    #[test]
    fn test_harmony_multiply_parts() {
        let txt = r#"
            Первая часть песни)
            |C|C/B|G7|Am7|
            |F G|G|
            Вторая часть песни)
            |F|F/Gb|Em|Dm|
            |F|Am|G|G7|
            Третья часть песни)
            |C|C7#11b13|C7m|C7m11|
            |F/G|A/G|G7|Gsus4|F|Fm|
            "#;
        let hrmn = SongHarmony::from_db_string(txt).unwrap();
        assert_eq!(hrmn.parts.len(), 3);
        assert_eq!(hrmn.parts[0].name, "Первая часть песни");
        assert_eq!(hrmn.parts[1].name, "Вторая часть песни");
        assert_eq!(hrmn.parts[2].name, "Третья часть песни");
        assert_eq!(hrmn.parts[0].bars.len(), 6);
        assert_eq!(hrmn.parts[1].bars.len(), 8);
        assert_eq!(hrmn.parts[2].bars.len(), 10);
        assert_eq!(hrmn.parts[0].bars[5].chords[0].to_string(), "G");
        assert_eq!(hrmn.parts[1].bars[3].chords[0].to_string(), "Dm");
        assert_eq!(hrmn.parts[2].bars[0].chords[0].to_string(), "C");
    }

    #[test]
    fn test_parse_bars_from_text_into_chord_elements() {
        let txt = "|C#m9|E9|C#/B|F#alt|";
        let bars = SongPart::parse_bars(txt).unwrap();
        assert_eq!(bars.len(), 4);
        assert_eq!(bars[0].chords[0].root, Note::CSharp);
        assert_eq!(bars[1].chords[0].color, Some("9".to_string()));
        assert_eq!(bars[3].chords[0].root, Note::FSharp);
        assert_eq!(bars[3].chords[0].color, Some("alt".to_string()));
    }

    #[test]
    fn test_new_song_part() {
        let part = SongPart::new("Куплет".to_string());
        assert_eq!(part.name, "Куплет");
        assert!(part.bars.is_empty());
        assert_eq!(part.bars.len(), 0);
    }

    #[test]
    fn test_from_text_simple() {
        let text = "Verse) | C Am | Dm G7 |";
        let part = SongPart::from_text(text).unwrap();
        assert_eq!(part.name, "Verse");
        assert_eq!(part.bars.len(), 2);
    }

    #[test]
    fn test_from_text_with_whitespace() {
        let text = "  Припев  )  | C G | Am F |  ";
        let part = SongPart::from_text(text).unwrap();
        assert_eq!(part.name, "Припев");
        assert_eq!(part.bars.len(), 2);
    }

    #[test]
    fn test_from_text_multiline() {
        let text = "Verse) | C Am | Dm G7 |\n| Em F | G C |";
        let part = SongPart::from_text(text).unwrap();
        assert_eq!(part.name, "Verse");
        assert_eq!(part.bars.len(), 4);
    }

    #[test]
    fn test_from_text_without_bars() {
        let text = "Intro) C G Am F";
        let part = SongPart::from_text(text).unwrap();
        assert_eq!(part.name, "Intro");
        assert_eq!(part.bars.len(), 1);
        let chord_strings: Vec<String> =
            part.bars[0].chords.iter().map(|c| c.to_string()).collect();
        assert_eq!(chord_strings, vec!["C", "G", "Am", "F"]);
    }

    #[test]
    fn test_from_text_no_separator() {
        let text = "Verse | C Am | Dm G7 |";
        let result = SongPart::from_text(text);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Нет разделителя ')'"));
    }

    #[test]
    fn test_from_text_empty_name() {
        let text = ") | C Am | Dm G7 |";
        let result = SongPart::from_text(text);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Имя части не может быть пустым")
        );
    }

    #[test]
    fn test_from_text_with_spaces_in_name() {
        let text = "  ) | C Am | Dm G7 |";
        let result = SongPart::from_text(text);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Имя части не может быть пустым")
        );
    }

    #[test]
    fn test_parse_bars_single_line_with_pipes() {
        let bars_text = "| C Am | Dm G7 | Em F |";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 3);
    }

    #[test]
    fn test_parse_bars_single_line_without_pipes() {
        let bars_text = "C Am Dm G7";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].chords.len(), 4);
    }

    #[test]
    fn test_parse_bars_multiple_lines() {
        let bars_text = "| C Am | Dm G7 |\n| Em F | G C |\n| Am G | F C |";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 6);
    }

    #[test]
    fn test_parse_bars_with_empty_lines() {
        let bars_text = "| C Am | Dm G7 |\n\n| Em F | G C |\n\n";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 4);
    }

    #[test]
    fn test_parse_bars_with_leading_trailing_pipes() {
        let bars_text = "| C Am | Dm G7 | Em F |";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 3);
    }

    #[test]
    fn test_parse_bars_with_mixed_format() {
        let bars_text = "| C Am | Dm G7\nEm F | G C |\nAm G | F C";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 6);
    }

    #[test]
    fn test_parse_bars_empty_text() {
        let bars_text = "";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert!(bars.is_empty());
    }

    #[test]
    fn test_parse_bars_whitespace_only() {
        let bars_text = "   \n\t   \n   ";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert!(bars.is_empty());
    }

    #[test]
    fn test_parse_bars_with_invalid_chord() {
        let bars_text = "| C Am | Invalid Xyz | Em F |";
        let result = SongPart::parse_bars(bars_text);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("такте #2"));
        assert!(error.contains("Invalid"));
    }

    #[test]
    fn test_parse_bars_preserves_order() {
        let bars_text = "| C | Dm | Em | F | G | Am |";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 6);
        let expected = ["C", "Dm", "Em", "F", "G", "Am"];
        for (i, bar) in bars.iter().enumerate() {
            assert_eq!(bar.chords[0].to_string(), expected[i]);
        }
    }

    #[test]
    fn test_parse_bars_with_chords_having_bass() {
        let bars_text = "| C/E | Dm/F | G7/B | Am/C |";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 4);
        assert_eq!(bars[0].chords[0].bass, Some(Note::ENat));
        assert_eq!(bars[1].chords[0].bass, Some(Note::FNat));
        assert_eq!(bars[2].chords[0].bass, Some(Note::BNat));
        assert_eq!(bars[3].chords[0].bass, Some(Note::CNat));
    }

    #[test]
    fn test_parse_bars_with_complex_chords() {
        let bars_text = "| Cmaj7 | Dm9 | G13 | Am7b5 |";
        let bars = SongPart::parse_bars(bars_text).unwrap();
        assert_eq!(bars.len(), 4);
        let expected = ["Cmaj7", "Dm9", "G13", "Am7b5"];
        for (i, bar) in bars.iter().enumerate() {
            assert_eq!(bar.chords[0].to_string(), expected[i]);
        }
    }

    #[test]
    fn test_parse_bars_counter_resets() {
        // Проверяем, что bar_counter правильно нумерует ошибки
        let bars_text = "| C | Dm |\n| Em | Invalid |\n| G | Am |";
        let result = SongPart::parse_bars(bars_text);
        assert!(result.is_err());
        let error = result.unwrap_err();
        // Invalid должен быть на 4-й позиции (C=1, Dm=2, Em=3, Invalid=4)
        assert!(error.contains("такте #4"));
    }

    #[test]
    fn test_parse_chords_in_bar_text_comparison() {
        let txt = "G7 C G7#11b13 C/E";
        let bar = Bar::from_chord_text(txt).unwrap();
        assert_eq!(bar.chords.len(), 4);
        let act_str: Vec<String> = bar.chords.iter().map(|c| c.to_string()).collect();
        let exp_str = vec!["G7", "C", "G7#11b13", "C/E"];
        assert_eq!(act_str, exp_str);
    }

    #[test]
    fn test_chords_alteration_in_bar() {
        let txt = "Am C";
        let expected = "Em G";
        let steps: i32 = 1;
        let bar = Bar::from_chord_text(txt).unwrap();
        let alt = bar.alter_chords(steps);
        assert_eq!(alt.to_string(), expected);
    }
}
