// dont use char because it's ambiguous
mod chars;
mod figure;
mod header;

use std::{collections::HashMap, path::Path, str::FromStr};

pub use header::RawHeader;
// pub use figure::FIGure;
pub use chars::FIGcharacter;
use thiserror::Error;

pub struct FIGfont {
    // temp until i write a Header struct that uses the enums
    pub header: RawHeader,
    // pub header: Header,
    pub comments: String,
    pub characters: HashMap<u32, FIGcharacter>,
}

#[derive(Debug, Error)]
pub enum FontLoadError {
    #[error("Attempted to load a font from an empty string")]
    EmptyString,
    #[error("Missing header")]
    MissingHeader,
    #[error("Failed to parse header: {0}")]
    HeaderError(#[from] header::HeaderParseError),
    #[error("Failed to parse character: {0}")]
    CharacterError(#[from] chars::CharacterParseError),
    #[error("IO Error: {0:?}")]
    IoError(#[from] std::io::Error),
}

impl FromStr for FIGfont {
    type Err = FontLoadError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(FontLoadError::EmptyString);
        }

        let lines: Vec<_> = s.lines().collect();

        let header: RawHeader = lines.first().ok_or(FontLoadError::MissingHeader)?.parse()?;
        let comments = lines[1..header.comment_lines as usize + 1].join("\n");
        let mut characters = HashMap::new();

        Self::read_required_characters(&lines, &header, &mut characters)?;

        Err(FontLoadError::EmptyString)
    }
}

impl FIGfont {
    fn read_required_characters(
        lines: &[&str],
        header: &RawHeader,
        map: &mut HashMap<u32, FIGcharacter>,
    ) -> Result<(), FontLoadError> {
        let offset = (header.comment_lines + 1) as usize;
        let char_height = header.height as usize;

        // ascii characters 32-126
        for i in 32..=126 {
            let code = i as u32;
            let idx = offset + ((i - 32) * char_height);

            let character = FIGcharacter::from_lines(&lines[idx..idx + char_height], code, header)?;
            map.insert(code, character);
        }

        Ok(())
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, FontLoadError> {
        let contents = std::fs::read_to_string(path)?;
        contents.parse()
    }
}
