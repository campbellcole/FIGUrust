use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::debug;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Convert {
        #[clap(short = 'f', long = "font-path", default_value = "default.frf")]
        font_path: PathBuf,
        #[clap(short = 'd', long = "font-dir", default_value_os_t = PathBuf::from("."))]
        output_dir: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    debug!("{:?}", args);
}
