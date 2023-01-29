use thiserror::Error;

use super::RawHeader;

pub struct FIGcharacter {
    pub code: u32,
    pub characters: Vec<String>,
    pub width: u16,
    pub baseline: u16,
}

#[derive(Debug, Error)]
pub enum CharacterParseError {
    #[error("Temp")]
    Temp,
}

impl FIGcharacter {
    pub fn from_lines(
        lines: &[&str],
        code: u32,
        header: &RawHeader,
    ) -> Result<Self, CharacterParseError> {
        let mut char_lines = vec![];
        let mut width = 0;
        for line in lines {
            let line = line.trim_end();
            if line.len() > width {
                width = line.len();
            }
            char_lines.push(line.to_string());
        }
        // TODO: left off here
    }
}
