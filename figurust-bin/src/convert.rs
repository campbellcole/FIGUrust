use std::{
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};

use figurust::figlet::{FIGfont, FontLoadError};
use ron::ser::PrettyConfig;
use thiserror::Error;

use crate::ConvertArgs;

#[derive(Debug, Error)]
pub enum ConvertFontError {
    #[error("Failed to load font file: {0}")]
    FontLoadError(#[from] FontLoadError),
    #[error("IO Error: {0:?}")]
    IoError(#[from] io::Error),
    #[error("Failed to serialize font: {0}")]
    SerializeError(#[from] ron::Error),
}

pub fn convert_font(
    ConvertArgs {
        input_file,
        output_dir,
        overwrite,
        pretty,
    }: &ConvertArgs,
) -> Result<PathBuf, ConvertFontError> {
    let font = FIGfont::from_file(input_file)?;

    let mut output_file_path = output_dir.to_path_buf();
    // SAFETY: if the font loaded, it has a file name
    output_file_path.push(input_file.file_name().unwrap());
    output_file_path.set_extension("ron");

    let mut open_opts = OpenOptions::new();

    if *overwrite {
        open_opts.create(true);
    } else {
        open_opts.create_new(true);
    }

    let output_file = open_opts
        .truncate(true)
        .write(true)
        .open(&output_file_path)?;

    if *pretty {
        ron::ser::to_writer_pretty(output_file, &font, PrettyConfig::default())?;
    } else {
        ron::ser::to_writer(output_file, &font)?;
    }

    Ok(output_file_path)
}
