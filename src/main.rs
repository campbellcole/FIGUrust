use std::{path::PathBuf, process::exit};

use clap::Parser;

mod font;
mod settings;
mod utils;

fn default_font_dir() -> PathBuf {
    dirs::data_dir()
        .expect("Could not find data directory, set --font-dir manually")
        .join("figurust")
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short = 'f', long = "font", default_value = "default.frf")]
    font: String,
    #[clap(short = 'd', long = "font-dir", default_value_os_t = default_font_dir())]
    font_directory: PathBuf,

    #[clap(short = 'c', long = "justify-center", default_value_t = false, action = clap::ArgAction::SetTrue)]
    justify_center: bool,
    #[clap(short = 'r', long = "justify-right", default_value_t = false, action = clap::ArgAction::SetTrue)]
    justify_right: bool,
    #[clap(short = 'l', long = "justify-left", default_value_t = false, action = clap::ArgAction::SetTrue)]
    justify_left: bool,
    #[clap(short = 'x', long = "justify-detect", default_value_t = false, action = clap::ArgAction::SetTrue)]
    justify_detect: bool,

    #[clap(short = 't', long = "use-terminal-width", default_value_t = false, action = clap::ArgAction::SetTrue)]
    use_terminal_width: bool,
    #[clap(short = 'w', long = "width", default_value = None)]
    width: Option<usize>,

    #[clap(short = 'p', long = "paragraph-mode", default_value_t = false, action = clap::ArgAction::SetTrue)]
    paragraph_mode: bool,
    #[clap(short = 'n', long = "normal-mode", default_value_t = false, action = clap::ArgAction::SetTrue)]
    normal_mode: bool,

    #[clap(short = 's', long = "smushing", default_value_t = false, action = clap::ArgAction::SetTrue)]
    smushing: bool,
    #[clap(short = 'S', long = "force-smushing", default_value_t = false, action = clap::ArgAction::SetTrue)]
    force_smushing: bool,
    #[clap(short = 'k', long = "kerning", default_value_t = false, action = clap::ArgAction::SetTrue)]
    kerning: bool,
    #[clap(short = 'W', long = "full-width", default_value_t = false, action = clap::ArgAction::SetTrue)]
    full_width: bool,

    #[clap(short = 'm', long = "layout-mode", default_value = None)]
    layout_mode: Option<u8>,

    #[clap(short = 'I', long = "info-code", default_value_t = -1)]
    info_code: i8,

    #[clap(short = 'L', long = "left-to-right", default_value_t = false, action = clap::ArgAction::SetTrue)]
    left_to_right: bool,
    #[clap(short = 'R', long = "right-to-left", default_value_t = false, action = clap::ArgAction::SetTrue)]
    right_to_left: bool,
    #[clap(short = 'X', long = "auto-direction", default_value_t = false, action = clap::ArgAction::SetTrue)]
    auto_direction: bool,
}

fn main() {
    let args = Args::parse();

    let settings = match settings::Settings::try_from(args) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if settings.info_code >= 0 {
        utils::print_info(&settings);
        exit(0);
    }

    println!("{:#?}", settings);
}
