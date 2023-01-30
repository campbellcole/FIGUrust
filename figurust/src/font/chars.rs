use thiserror::Error;

use super::RawHeader;

#[derive(Debug)]
pub struct FIGcharacter {
    pub code: u32,
    pub char_lines: Vec<String>,
    pub width: usize,
}

#[derive(Debug, Error)]
pub enum CharacterParseError {
    #[error("Temp")]
    _Temp,
}

impl FIGcharacter {
    pub fn from_lines(
        lines: &[&str],
        code: u32,
        header: &RawHeader,
    ) -> Result<Self, CharacterParseError> {
        let mut char_lines = vec![];
        let mut width = 0;
        for (x, line) in lines.iter().enumerate() {
            let line_width = if x == lines.len() - 1 && header.height != 1 {
                // the last line of multiline characters have @@ at the end
                line.len() - 2
            } else {
                // all other lines have @ at the end
                line.len() - 1
            };

            if line_width > width {
                width = line_width;
            }

            char_lines.push(line[..line_width].replace(header.hardblank, " "));
        }
        Ok(Self {
            code,
            char_lines,
            width,
        })
    }
}
