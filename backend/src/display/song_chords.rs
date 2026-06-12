use crate::music_logic::{Bar, SongHarmony};

impl SongHarmony {
    pub fn formated_display(&self) -> String {
        if self.parts.is_empty() {
            return String::new();
        }

        let mut bar_result = String::new();

        for part in &self.parts {
            bar_result.push_str(&format!("{})\n", part.name));

            for (i, bar) in part.bars.iter().enumerate() {
                let mut chord_result = String::new();

                for (j, chord) in bar.chords.iter().enumerate() {
                    let max_bar_width = self.max_bar_width();
                    let chord_numbers = bar.chords.len();
                    let symbols_for_chord = max_bar_width / chord_numbers;

                    let chord_string =
                        format!("{:<width$}", &chord.to_string(), width = symbols_for_chord);

                    if (j) < chord_numbers {
                        chord_result.push_str(&chord_string);
                    }
                }

                if (i + 1) % 4 == 1 {
                    chord_result.insert(0, '|');
                }

                let bar_str = format!("{}", chord_result);

                bar_result.push_str(&bar_str);

                if (i + 1) % 4 != 0 && i < part.bars.len() - 1 {
                    bar_result.push('|');
                }

                if (i + 1) % 4 == 0 {
                    bar_result.push('|');
                    bar_result.push('\n');
                }
            }
        }

        bar_result
    }

    pub fn max_bar_width(&self) -> usize {
        let c_w: usize = self.max_chord_width();
        let c_n: usize = self.max_chords_in_every_bar();
        let result = c_w * c_n;
        result
    }

    pub fn max_chord_width(&self) -> usize {
        let real_width: usize = self
            .parts
            .iter()
            .flat_map(|part| part.bars.iter())
            .flat_map(|bar| bar.chords.iter())
            .map(|chord| chord.to_string().len())
            .max()
            .unwrap_or(0);

        let safe_width = real_width + 1;
        safe_width
    }

    pub fn max_chords_in_every_bar(&self) -> usize {
        self.parts
            .iter()
            .flat_map(|part| part.bars.iter())
            .map(|bar| bar.chords.len())
            .max()
            .unwrap_or(0)
    }
}

impl std::fmt::Display for SongHarmony {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formated_display())
    }
}

impl std::fmt::Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chord_strings: Vec<String> = self.chords.iter().map(|c| c.to_string()).collect();
        write!(f, "{}", chord_strings.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song_display() {
        let txt = r#"
            Куплет )
            |C|Am|D7#11|Dm Db|
            |F|G|C C/B |C/A C/G|
            Припев)
            |E7#11 F9| C/E Am911| Bm| Gmaj Gmaj7 Gsus2 Gmaj7|
            "#;
        let hrmn = SongHarmony::from_db_string(txt).unwrap();
        println!("{}", hrmn);
    }

    #[test]
    fn test_max_point_somained_funcs() {
        let txt = r#"
                    Куплет )
                    |C|Am|D7#11|Dm|
                    |F|G|C C/B C/A C/G| G|
                    "#;
        let hrmn = SongHarmony::from_db_string(txt).unwrap();
        let bar_length = SongHarmony::max_bar_width(&hrmn);
        let chord_length = SongHarmony::max_chord_width(&hrmn);
        let chord_number = SongHarmony::max_chords_in_every_bar(&hrmn);

        let bar_expected: usize = 24;
        let chord_l_exp: usize = 6;
        let chord_n_exp: usize = 4;

        assert_eq!(bar_length, bar_expected);
        assert_eq!(chord_length, chord_l_exp);
        assert_eq!(chord_number, chord_n_exp);
    }
}
