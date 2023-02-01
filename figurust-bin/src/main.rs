use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use log::{debug, error, info};

pub mod convert;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Convert a .flf font to a .ron font
    Convert(ConvertArgs),
}

#[derive(Debug, Args)]
pub struct ConvertArgs {
    #[clap(short = 'i', long = "input-file")]
    input_file: PathBuf,
    #[clap(short = 'o', long = "output-dir", default_value_os_t = PathBuf::from("."))]
    output_dir: PathBuf,
    #[clap(short = 'f', long = "force", default_value_t = false, action = clap::ArgAction::SetTrue)]
    overwrite: bool,
    #[clap(short = 'p', long = "pretty", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pretty: bool,
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    if let Some(command) = &args.command {
        match command {
            Commands::Convert(convert) => match convert::convert_font(convert) {
                Ok(path) => info!(
                    "Converted font: {} -> {}",
                    convert.input_file.display(),
                    path.display()
                ),
                Err(e) => error!("Failed to convert font: {}", e),
            },
        }
    }

    debug!("{:?}", args);
}
