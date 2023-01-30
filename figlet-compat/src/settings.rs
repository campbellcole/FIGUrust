use std::path::PathBuf;

use thiserror::Error;

use crate::Args;

#[derive(Debug, Default)]
pub enum Justify {
    Left,
    Center,
    Right,
    #[default]
    Auto,
}

#[derive(Debug, Default)]
pub enum Mode {
    Paragraph,
    #[default]
    Normal,
}

#[derive(Debug, Default)]
pub enum Spacing {
    #[default]
    Smushing,
    ForceSmushing,
    Kerning,
    FullWidth,
}

#[derive(Debug, Default)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    #[default]
    Auto,
}

#[derive(Debug)]
pub struct Settings {
    pub font: String,
    pub font_directory: PathBuf,

    pub justify: Justify,

    pub width: usize,

    pub mode: Mode,

    pub spacing: Spacing,

    pub layout_mode: Option<u8>,

    pub info_code: i8,

    pub direction: Direction,
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("cannot set multiple {0} options")]
    ConflictingOptions(&'static str),
    #[error("Attempted to use --use-terminal-width outside of a terminal")]
    NoTerminalSize,
}

fn assert_none_true(
    name: &'static str,
    vals: impl IntoIterator<Item = bool>,
) -> Result<(), SettingsError> {
    vals.into_iter()
        .all(|x| !x)
        .then_some(())
        .ok_or(SettingsError::ConflictingOptions(name))
}

impl TryFrom<Args> for Settings {
    type Error = SettingsError;
    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let mut justify = Justify::default();
        if value.justify_detect {
            justify = Justify::Auto;
            assert_none_true(
                "justify",
                [
                    value.justify_left,
                    value.justify_right,
                    value.justify_center,
                ],
            )?;
        } else if value.justify_center {
            justify = Justify::Center;
            assert_none_true(
                "justify",
                [
                    value.justify_left,
                    value.justify_right,
                    value.justify_detect,
                ],
            )?;
        } else if value.justify_right {
            justify = Justify::Right;
            assert_none_true(
                "justify",
                [
                    value.justify_left,
                    value.justify_center,
                    value.justify_detect,
                ],
            )?;
        } else if value.justify_left {
            justify = Justify::Left;
            assert_none_true(
                "justify",
                [
                    value.justify_right,
                    value.justify_center,
                    value.justify_detect,
                ],
            )?;
        }

        let mut width = 80;
        if value.use_terminal_width {
            width = termsize::get()
                .map(|dim| dim.cols as usize)
                .ok_or(SettingsError::NoTerminalSize)?;
            assert_none_true("width", [value.width.is_some()])?;
        } else if let Some(w) = value.width {
            width = w;
            assert_none_true("width", [value.use_terminal_width])?;
        }

        let mut mode = Mode::default();
        if value.paragraph_mode {
            mode = Mode::Paragraph;
            assert_none_true("mode", [value.normal_mode])?;
        } else if value.normal_mode {
            mode = Mode::Normal;
            assert_none_true("mode", [value.paragraph_mode])?;
        }

        let mut spacing = Spacing::default();
        if value.smushing {
            spacing = Spacing::Smushing;
            assert_none_true(
                "spacing",
                [value.force_smushing, value.kerning, value.full_width],
            )?;
        } else if value.force_smushing {
            spacing = Spacing::ForceSmushing;
            assert_none_true("spacing", [value.smushing, value.kerning, value.full_width])?;
        } else if value.kerning {
            spacing = Spacing::Kerning;
            assert_none_true(
                "spacing",
                [value.smushing, value.force_smushing, value.full_width],
            )?;
        } else if value.full_width {
            spacing = Spacing::FullWidth;
            assert_none_true(
                "spacing",
                [value.smushing, value.force_smushing, value.kerning],
            )?;
        }

        let mut direction = Direction::default();
        if value.auto_direction {
            direction = Direction::Auto;
            assert_none_true("direction", [value.left_to_right, value.right_to_left])?;
        } else if value.left_to_right {
            direction = Direction::LeftToRight;
            assert_none_true("direction", [value.auto_direction, value.right_to_left])?;
        } else if value.right_to_left {
            direction = Direction::RightToLeft;
            assert_none_true("direction", [value.auto_direction, value.left_to_right])?;
        }

        Ok(Self {
            font: value.font,
            font_directory: value.font_directory,
            justify,
            width,
            mode,
            spacing,
            layout_mode: value.layout_mode,
            info_code: value.info_code,
            direction,
        })
    }
}