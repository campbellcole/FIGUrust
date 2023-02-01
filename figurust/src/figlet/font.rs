use std::{collections::HashMap, path::Path, str::FromStr};

use itertools::Itertools;
use thiserror::Error;

use crate::settings::{Settings, Spacing};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
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
    HeaderError(#[from] HeaderParseError),
    #[error("Failed to parse character: {0}")]
    CharacterError(#[from] CharacterParseError),
    #[error("IO Error: {0:?}")]
    IoError(#[from] std::io::Error),
}

impl FromStr for FIGfont {
    type Err = FontLoadError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(FontLoadError::EmptyString);
        }

        let mut lines: Vec<_> = s.lines().collect();

        let header: RawHeader = lines.first().ok_or(FontLoadError::MissingHeader)?.parse()?;
        let comments = lines[1..header.comment_lines as usize + 1].join("\n");
        let mut characters = HashMap::new();

        // this function takes the characters out of the lines vector
        Self::read_required_characters(&mut lines, &header, &mut characters)?;

        Ok(Self {
            header,
            comments,
            characters,
        })
    }
}

// static methods
impl FIGfont {
    fn read_required_characters(
        lines: &mut [&str],
        header: &RawHeader,
        map: &mut HashMap<u32, FIGcharacter>,
    ) -> Result<(), FontLoadError> {
        let offset = (header.comment_lines + 1) as usize;
        let char_height = header.height as usize;

        // ascii characters 32-126
        for i in 32..=126 {
            let idx = offset + ((i - 32) * char_height);

            let character = FIGcharacter::from_lines(&mut lines[idx..idx + char_height], header)?;
            map.insert(i as u32, character);
        }

        Ok(())
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, FontLoadError> {
        let contents = std::fs::read_to_string(path)?;
        contents.parse()
    }
}

#[derive(Debug, Error)]
pub enum FontConvertError {
    #[error("Font does not contain character: '{0}'")]
    MissingCharacter(char),
}

// instance methods
impl FIGfont {
    pub fn convert(
        &self,
        content: impl AsRef<str>,
        settings: &Settings,
    ) -> Result<String, FontConvertError> {
        let content = content.as_ref();
        let mut char_lines = vec![];
        let mut output = String::new();

        for line in content.lines() {
            let mut chars = vec![];
            for c in line.chars() {
                let code = c as u32;
                let Some(character) = self.characters.get(&code) else {
                    return Err(FontConvertError::MissingCharacter(c));
                };
                chars.push(character);
            }
            char_lines.push(chars);
        }

        match settings.spacing {
            Spacing::FullWidth => {
                for char_line in char_lines {
                    for y in 0..self.header.height {
                        for character in &char_line {
                            output.push_str(&character.char_lines[y as usize]);
                        }
                        output.push('\n');
                    }
                }
            }
            Spacing::Smushing => {
                for char_line in char_lines {
                    for y in 0..self.header.height {
                        char_line.first().unwrap().join_line(
                            &mut output,
                            y,
                            &settings.spacing,
                            None,
                        );
                        for (prev, character) in char_line.iter().tuple_windows() {
                            character.join_line(&mut output, y, &settings.spacing, Some(prev));
                        }
                        output.push('\n');
                    }
                }
            }
            _ => unimplemented!("Spacing: {:?}", settings.spacing),
        }

        Ok(output)
    }
}
