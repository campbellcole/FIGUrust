use std::mem::take;

use thiserror::Error;

use crate::settings::Spacing;

use super::RawHeader;

#[derive(Debug, Serialize, Deserialize)]
pub struct FIGcharacter {
    pub char_lines: Vec<String>,
    pub width: usize,
}

#[derive(Debug, Error)]
pub enum CharacterParseError {
    #[error("Temp")]
    _Temp,
}

// i am envisioning a much less readable version of this
// but this uses a small amount of memory and is easy to read

const TOP_CMP: &str = "_";
const TOP_HEIRARCHY: &str = "|/\\[]{}()<>";

const BARLESS_CMP: &str = "|";
const BARLESS_HEIRARCHY: &str = "/\\[]{}()<>";

const SLASHLESS_CMP: &str = "/\\";
const SLASHLESS_HEIRARCHY: &str = "[]{}()<>";

const BRACKETLESS_CMP: &str = "[]";
const BRACKETLESS_HEIRARCHY: &str = "{}()<>";

const CURLYLESS_CMP: &str = "{}";
const CURLYLESS_HEIRARCHY: &str = "()<>";

const PARENLESS_CMP: &str = "()";
const PARENLESS_HEIRARCHY: &str = "<>";

impl FIGcharacter {
    pub fn join_line(
        &self,
        output: &mut String,
        idx: impl Into<usize>,
        spacing: &Spacing,
        prev: Option<&FIGcharacter>,
    ) {
        let idx: usize = idx.into();
        assert!(idx < self.char_lines.len());
        match spacing {
            Spacing::FullWidth => {
                output.push_str(&self.char_lines[idx]);
            }
            Spacing::Smushing => {
                if let Some(last_char) = prev {
                    if last_char.width < 2 {
                        self.join_line(output, idx, &Spacing::FullWidth, prev);
                    } else {
                        // remove space from the beginning of the line
                        let line = &self.char_lines[idx][1..];

                        let lch = last_char.char_lines[idx].chars().last().unwrap();
                        let rch = line.chars().next().unwrap();

                        let mut res: Option<char> = None;

                        if lch == ' ' || rch == ' ' {
                            res = Some(' ');
                        }

                        if let Some(res) = res {
                            output.push(res);
                            output.push_str(line);
                        } else {
                            output.push_str(&self.char_lines[idx]);
                        }
                    }
                } else {
                    self.join_line(output, idx, &Spacing::FullWidth, prev);
                }
            }
            _ => unimplemented!("Spacing: {:?}", spacing),
        }
    }

    pub fn from_lines(lines: &mut [&str], header: &RawHeader) -> Result<Self, CharacterParseError> {
        let mut char_lines = vec![];
        let mut width = 0;
        let len = lines.len();
        for (x, line) in lines.iter_mut().enumerate() {
            let line_width = if x == len - 1 && header.height != 1 {
                // the last line of multiline characters have @@ at the end
                line.len() - 2
            } else {
                // all other lines have @ at the end
                line.len() - 1
            };

            if line_width > width {
                width = line_width;
            }
            let owned = take(line)[..line_width].to_owned();
            char_lines.push(owned);
        }
        Ok(Self { char_lines, width })
    }
}
