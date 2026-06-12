use std::fmt;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Note {
    CNat,
    GNat,
    DNat,
    ANat,
    ENat,
    BNat,
    FSharp,
    CSharp,
    AFlat,
    EFlat,
    BFlat,
    FNat,
}

impl Note {
    const CIRCLE: [Note; 12] = [
        Note::CNat,
        Note::GNat,
        Note::DNat,
        Note::ANat,
        Note::ENat,
        Note::BNat,
        Note::FSharp,
        Note::CSharp,
        Note::AFlat,
        Note::EFlat,
        Note::BFlat,
        Note::FNat,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Note::CNat => "C",
            Note::GNat => "G",
            Note::DNat => "D",
            Note::ANat => "A",
            Note::ENat => "E",
            Note::BNat => "B",
            Note::FSharp => "F#",
            Note::CSharp => "C#",
            Note::AFlat => "Ab",
            Note::EFlat => "Eb",
            Note::BFlat => "Bb",
            Note::FNat => "F",
        }
    }

    //НУЖНО КУПИРОВАТЬ РАЗНИЦУ ЗАГЛАВНЫХ/СТРОЧНЫХ БУКВ ТОНИКИ И ЗНАКА

    pub fn from_db_string(s: &str) -> Result<Self, &str> {
        let cleaned = s.trim();

        match cleaned {
            "C" | "B#" => Ok(Note::CNat),
            "G" => Ok(Note::GNat),
            "D" => Ok(Note::DNat),
            "A" => Ok(Note::ANat),
            "E" | "Fb" => Ok(Note::ENat),
            "B" | "Cb" => Ok(Note::BNat),
            "F#" | "Gb" => Ok(Note::FSharp),
            "C#" | "Db" => Ok(Note::CSharp),
            "Ab" | "G#" => Ok(Note::AFlat),
            "Eb" | "D#" => Ok(Note::EFlat),
            "Bb" | "A#" => Ok(Note::BFlat),
            "F" => Ok(Note::FNat),
            _ => Err("Неизвестная нота"),
        }
    }

    //АЛЬТЕРАЦИЯ
    pub fn get_note_index(&self) -> usize {
        Self::CIRCLE.iter().position(|&k| k == *self).unwrap_or(0)
    }

    pub fn next_note(&self) -> Self {
        let current_index = self.get_note_index();
        let next_index = (current_index + 1) % Self::CIRCLE.len();
        Self::CIRCLE[next_index]
    }

    pub fn alter_by_steps(&self, steps: i32) -> Self {
        let current_index = self.get_note_index();
        let len = Self::CIRCLE.len() as i32;
        let new_index = (current_index as i32 + steps).rem_euclid(len) as usize;
        Self::CIRCLE[new_index]
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str_correct_for_each_note() {
        assert_eq!(Note::CNat.as_str(), "C");
        assert_eq!(Note::GNat.as_str(), "G");
        assert_eq!(Note::DNat.as_str(), "D");
        assert_eq!(Note::ANat.as_str(), "A");
        assert_eq!(Note::ENat.as_str(), "E");
        assert_eq!(Note::BNat.as_str(), "B");
        assert_eq!(Note::FSharp.as_str(), "F#");
        assert_eq!(Note::CSharp.as_str(), "C#");
        assert_eq!(Note::FNat.as_str(), "F");
        assert_eq!(Note::BFlat.as_str(), "Bb");
        assert_eq!(Note::EFlat.as_str(), "Eb");
        assert_eq!(Note::AFlat.as_str(), "Ab");
    }

    #[test]
    fn display_uses_as_str() {
        assert_eq!(format!("{}", Note::CNat), "C");
        assert_eq!(format!("{}", Note::FSharp), "F#");
        assert_eq!(format!("{}", Note::AFlat), "Ab");
    }

    #[test]
    fn test_from_db_string_valid_inputs() {
        assert_eq!(Note::from_db_string("C").unwrap(), Note::CNat);
        assert_eq!(Note::from_db_string("B#").unwrap(), Note::CNat);
        assert_eq!(Note::from_db_string("G").unwrap(), Note::GNat);
        assert_eq!(Note::from_db_string("D").unwrap(), Note::DNat);
        assert_eq!(Note::from_db_string("A").unwrap(), Note::ANat);
        assert_eq!(Note::from_db_string("E").unwrap(), Note::ENat);
        assert_eq!(Note::from_db_string("Fb").unwrap(), Note::ENat);
        assert_eq!(Note::from_db_string("B").unwrap(), Note::BNat);
        assert_eq!(Note::from_db_string("Cb").unwrap(), Note::BNat);
        assert_eq!(Note::from_db_string("F").unwrap(), Note::FNat);
        assert_eq!(Note::from_db_string("F#").unwrap(), Note::FSharp);
        assert_eq!(Note::from_db_string("Gb").unwrap(), Note::FSharp);
        assert_eq!(Note::from_db_string("C#").unwrap(), Note::CSharp);
        assert_eq!(Note::from_db_string("Db").unwrap(), Note::CSharp);
        assert_eq!(Note::from_db_string("Bb").unwrap(), Note::BFlat);
        assert_eq!(Note::from_db_string("A#").unwrap(), Note::BFlat);
        assert_eq!(Note::from_db_string("Eb").unwrap(), Note::EFlat);
        assert_eq!(Note::from_db_string("D#").unwrap(), Note::EFlat);
        assert_eq!(Note::from_db_string("Ab").unwrap(), Note::AFlat);
        assert_eq!(Note::from_db_string("G#").unwrap(), Note::AFlat);
        assert_eq!(Note::from_db_string(" C").unwrap(), Note::CNat);
        assert_eq!(Note::from_db_string("Db ").unwrap(), Note::CSharp);
        assert_eq!(Note::from_db_string(" G# ").unwrap(), Note::AFlat);
    }

    #[test]
    fn test_from_db_invalid_inputs() {
        assert!(Note::from_db_string("H").is_err());
        assert!(Note::from_db_string("0").is_err());
        assert!(Note::from_db_string("C##").is_err());
        assert!(Note::from_db_string("Fbb").is_err());
        assert!(Note::from_db_string("Bb#").is_err());
        assert!(Note::from_db_string("").is_err());
        assert!(Note::from_db_string("    ").is_err());
    }

    #[test]
    fn test_get_note_index_returns_correct_positions() {
        assert_eq!(Note::CNat.get_note_index(), 0);
        assert_eq!(Note::GNat.get_note_index(), 1);
        assert_eq!(Note::DNat.get_note_index(), 2);
        assert_eq!(Note::ANat.get_note_index(), 3);
        assert_eq!(Note::ENat.get_note_index(), 4);
        assert_eq!(Note::BNat.get_note_index(), 5);
        assert_eq!(Note::FSharp.get_note_index(), 6);
        assert_eq!(Note::CSharp.get_note_index(), 7);
        assert_eq!(Note::AFlat.get_note_index(), 8);
        assert_eq!(Note::EFlat.get_note_index(), 9);
        assert_eq!(Note::BFlat.get_note_index(), 10);
        assert_eq!(Note::FNat.get_note_index(), 11);
    }

    #[test]
    fn test_next_note_moves_forward_in_circle_of_fifths() {
        // Проверяем последовательность
        assert_eq!(Note::CNat.next_note(), Note::GNat);
        assert_eq!(Note::GNat.next_note(), Note::DNat);
        assert_eq!(Note::DNat.next_note(), Note::ANat);
        assert_eq!(Note::ANat.next_note(), Note::ENat);
        assert_eq!(Note::ENat.next_note(), Note::BNat);
        assert_eq!(Note::BNat.next_note(), Note::FSharp);
        assert_eq!(Note::FSharp.next_note(), Note::CSharp);
        assert_eq!(Note::CSharp.next_note(), Note::AFlat);
        assert_eq!(Note::AFlat.next_note(), Note::EFlat);
        assert_eq!(Note::EFlat.next_note(), Note::BFlat);
        assert_eq!(Note::BFlat.next_note(), Note::FNat);
        // Замыкание круга
        assert_eq!(Note::FNat.next_note(), Note::CNat);
    }

    // Тесты для alter_by_steps - альтерация на разное количество шагов
    #[test]
    fn test_alter_by_steps_positive_steps() {
        let note = Note::CNat;
        assert_eq!(note.alter_by_steps(1), Note::GNat); // C +1 = G
        assert_eq!(note.alter_by_steps(2), Note::DNat); // C +2 = D
        assert_eq!(note.alter_by_steps(3), Note::ANat); // C +3 = A
        assert_eq!(note.alter_by_steps(4), Note::ENat); // C +4 = E
        assert_eq!(note.alter_by_steps(5), Note::BNat); // C +5 = B
        assert_eq!(note.alter_by_steps(6), Note::FSharp); // C +6 = F#
        assert_eq!(note.alter_by_steps(7), Note::CSharp); // C +7 = C#
        assert_eq!(note.alter_by_steps(8), Note::AFlat); // C +11 = Ab
        assert_eq!(note.alter_by_steps(9), Note::EFlat); // C +10 = Eb
        assert_eq!(note.alter_by_steps(10), Note::BFlat); // C +9 = Bb
        assert_eq!(note.alter_by_steps(11), Note::FNat); // C +8 = F
        assert_eq!(note.alter_by_steps(12), Note::CNat); // C +12 = C (полный круг)
    }

    #[test]
    fn test_alter_by_steps_negative_steps() {
        let note = Note::CNat;
        assert_eq!(note.alter_by_steps(-1), Note::FNat); // C -4 = F
        assert_eq!(note.alter_by_steps(-2), Note::BFlat); // C -3 = Bb
        assert_eq!(note.alter_by_steps(-3), Note::EFlat); // C -2 = Eb
        assert_eq!(note.alter_by_steps(-4), Note::AFlat); // C -1 = Ab
        assert_eq!(note.alter_by_steps(-5), Note::CSharp); // C -5 = C#
        assert_eq!(note.alter_by_steps(-6), Note::FSharp); // C -6 = F#
        assert_eq!(note.alter_by_steps(-7), Note::BNat); // C -7 = B
        assert_eq!(note.alter_by_steps(-8), Note::ENat); // C -8 = E
        assert_eq!(note.alter_by_steps(-9), Note::ANat); // C -9 = A
        assert_eq!(note.alter_by_steps(-10), Note::DNat); // C -10 = D
        assert_eq!(note.alter_by_steps(-11), Note::GNat); // C -11 = G
        assert_eq!(note.alter_by_steps(-12), Note::CNat); // C -12 = C (полный круг)
    }

    #[test]
    fn test_alter_by_steps_different_starting_notes() {
        // Тестируем с разных нот
        assert_eq!(Note::FSharp.alter_by_steps(1), Note::CSharp); // F# +1 = C#
        assert_eq!(Note::BFlat.alter_by_steps(2), Note::CNat); // Bb +2 = Eb
        assert_eq!(Note::ENat.alter_by_steps(-1), Note::ANat); // E -1 = A
    }

    #[test]
    fn test_alter_by_steps_large_steps() {
        let note = Note::CNat;
        // Большие шаги (кратные кругу)
        assert_eq!(note.alter_by_steps(12), Note::CNat); // +12 = C
        assert_eq!(note.alter_by_steps(24), Note::CNat); // +24 = C
        assert_eq!(note.alter_by_steps(13), Note::GNat); // +13 = G (+1 после круга)
        assert_eq!(note.alter_by_steps(25), Note::GNat); // +25 = G (+1 после двух кругов)
        assert_eq!(note.alter_by_steps(-12), Note::CNat); // -12 = C
        assert_eq!(note.alter_by_steps(-13), Note::FNat); // -13 = Ab (-1 после круга)
    }
}
