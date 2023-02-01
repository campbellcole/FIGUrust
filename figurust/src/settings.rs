// pub trait FontSettings {
//     fn get_font_name(&self) -> &str;
//     fn get_font_directory(&self) -> &Path;
//     fn get_justify(&self) -> Justify;
//     fn get_width(&self) -> usize;
//     fn get_mode(&self) -> Mode;
//     fn get_spacing(&self) -> Spacing;
// }

use std::{
    io,
    path::PathBuf,
    process::{Command, Stdio},
    string::FromUtf8Error,
};

use execute::Execute;
use thiserror::Error;

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

    pub overlap: bool,

    pub layout_mode: Option<u8>,

    pub info_code: i8,

    pub direction: Direction,
}

#[derive(Debug, Error)]
pub enum DefaultSettingsError {
    #[error("Io Error: {0:?}")]
    IoError(#[from] io::Error),
    #[error("figlet exited with code {0}")]
    NonZeroExitCode(i32),
    #[error("Utf8 Error: {0:?}")]
    Utf8Error(#[from] FromUtf8Error),
}

pub fn figlet_default_settings() -> Result<Settings, DefaultSettingsError> {
    let mut figlet_cmd = Command::new("figlet");
    figlet_cmd.args(["-I", "2"]).stdout(Stdio::piped());

    let output = figlet_cmd.execute_output()?;

    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            return Err(DefaultSettingsError::NonZeroExitCode(exit_code));
        }
    }

    let font_directory = String::from_utf8(output.stdout)?;
    // SAFETY: if code is zero there will be exactly one line
    let font_directory = PathBuf::from(font_directory.lines().next().unwrap());

    Ok(Settings {
        font: "standard".to_string(),
        font_directory,
        justify: Justify::default(),
        width: 80,
        mode: Mode::default(),
        spacing: Spacing::default(),
        overlap: false,
        layout_mode: None,
        info_code: 0,
        direction: Direction::default(),
    })
}
