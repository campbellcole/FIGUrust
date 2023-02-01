use std::{io::Write, path::PathBuf, process::exit};

use clap::{command, Parser};
use log::{debug, error, info, Level};
use owo_colors::OwoColorize;
use settings::FigletSettings;

use crate::{
    consts::*,
    settings::{Justification, ParagraphMode, RightToLeft},
};

pub mod consts;
mod settings;
mod utils;

fn default_font_dir() -> PathBuf {
    dirs::data_dir()
        .expect("Could not find data directory, set --font-dir manually")
        .join("figurust")
}

fn print_usage(bin_name: &str) {
    println!("Usage: {bin_name} [ -cklnoprstvxLRSWX ] [ -d fontdirectory ]");
    println!("      [ -f fontfile ] [ -m smushmode ] [ -w outputwidth ]");
    println!("      [ -I infocode ] [ message ]")
}

fn missing_arg(bin_name: &str, arg: &str) -> ! {
    error!("Missing argument for {arg}");
    print_usage(bin_name);
    exit(1);
}

pub fn main() {
    env_logger::builder()
        .format(|buf, record| {
            let level = match record.level() {
                Level::Error => record.level().red().to_string(),
                Level::Warn => record.level().yellow().to_string(),
                Level::Info => record.level().green().to_string(),
                Level::Debug => record.level().blue().to_string(),
                Level::Trace => record.level().purple().to_string(),
            };
            writeln!(buf, "[{}] - {}", level, record.args())
        })
        .init();
    info!("Starting figurust");

    let settings = from_args();

    debug!("Settings: {:#?}", settings);
}

pub fn from_args() -> FigletSettings {
    let args: Vec<_> = std::env::args().collect();

    let mut bin_name = args.first().unwrap().as_str();
    if bin_name.contains('/') {
        bin_name = bin_name.split('/').last().unwrap();
    } else if bin_name.contains('\\') {
        bin_name = bin_name.split('\\').last().unwrap();
    }

    println!("bin_name: {bin_name}");

    // enabling this breaks some compatibility with figlet
    // figlet uses 0 for any invalid arguments
    // but with this argument we will print an error and exit
    let mut show_errors = false;

    let mut font_name = "standard.flf".to_string();
    let mut font_directory = default_font_dir();
    let mut smush_override = SMO_NO;
    let mut smush_mode = None::<Smushmode>;
    let mut justification = Justification::Auto;
    let mut right_to_left = RightToLeft::Detect;
    let mut paragraph = ParagraphMode::Normal;
    let mut infocode: i8 = -1;
    let mut width: u16 = DEFAULT_COLUMNS;

    let mut message: Option<String> = None;

    let mut consumed_next = false;

    for (x, arg) in args[1..].iter().enumerate() {
        if consumed_next {
            consumed_next = false;
            continue;
        }
        // adjust for the slice
        let x = x + 1;
        if arg.len() > 1 && arg.starts_with('-') {
            match &arg[1..] {
                // our own arguments
                "e" => {
                    show_errors = true;
                }
                // this would trigger both -n and -e in figlet
                // but that would cause an error because
                // -e is not a valid argument, so this won't
                // break compatibility
                "ne" => {
                    show_errors = false;
                }

                // figlet arguments
                "X" => {
                    right_to_left = RightToLeft::Detect;
                }
                "L" => {
                    right_to_left = RightToLeft::Left;
                }
                "R" => {
                    right_to_left = RightToLeft::Right;
                }
                "x" => {
                    justification = Justification::Auto;
                }
                "l" => {
                    justification = Justification::Left;
                }
                "c" => {
                    justification = Justification::Center;
                }
                "r" => {
                    justification = Justification::Right;
                }
                "p" => {
                    paragraph = ParagraphMode::Paragraph;
                }
                "n" => {
                    paragraph = ParagraphMode::Normal;
                }
                "s" => {
                    smush_override = SMO_NO;
                    smush_mode = None;
                }
                "k" => {
                    smush_mode = Some(SM_KERN);
                    smush_override = SMO_YES;
                }
                "S" => {
                    smush_mode = Some(SM_SMUSH);
                    smush_override = SMO_FORCE;
                }
                "o" => {
                    smush_mode = Some(SM_SMUSH);
                    smush_override = SMO_YES;
                }
                "W" => {
                    smush_mode = Some(SM_FULLWIDTH);
                    smush_override = SMO_YES;
                }
                "t" => {
                    let term_size = termsize::get();
                    if let Some(term_size) = term_size {
                        width = term_size.cols;
                    } else {
                        error!("-t option requires a terminal");
                    }
                }
                "v" => {
                    infocode = 0;
                }
                "I" => {
                    match args.get(x + 1).map(|s| s.parse()) {
                        Some(Ok(next)) => {
                            infocode = next;
                        }
                        Some(Err(err)) => {
                            if show_errors {
                                error!("Invalid argument for -I: {}", err);
                                exit(1);
                            } else {
                                infocode = 0;
                            }
                        }
                        None => {
                            missing_arg(bin_name, "-I");
                        }
                    };
                    consumed_next = true;
                }
                "m" => {
                    match args.get(x + 1).map(|s| s.parse::<i32>()) {
                        Some(Ok(mode)) => {
                            if mode < -1 {
                                smush_override = SMO_NO;
                                continue;
                            }
                            if mode == 0 {
                                smush_mode = Some(SM_KERN);
                            } else if mode == -1 {
                                smush_mode = Some(SM_FULLWIDTH);
                            } else {
                                smush_mode = Some((mode & 63) | SM_SMUSH);
                            }
                            smush_override = SMO_YES;
                        }
                        Some(Err(err)) => {
                            if show_errors {
                                error!("Invalid argument for -m: {}", err);
                                exit(1);
                            } else {
                                smush_override = 0;
                            }
                        }
                        None => {
                            missing_arg(bin_name, "-m");
                        }
                    };
                    consumed_next = true;
                }
                "w" => {
                    match args.get(x + 1).map(|s| s.parse::<u16>()) {
                        Some(Ok(next)) => {
                            if next > 0 {
                                width = next;
                            }
                        }
                        Some(Err(err)) => {
                            if show_errors {
                                error!("Invalid argument for -w: {}", err);
                                exit(1);
                            }
                            // no else block because atoi defaults to 0
                            // and any values < 1 are ignored by figlet
                        }
                        None => {
                            missing_arg(bin_name, "-w");
                        }
                    };
                    consumed_next = true;
                }
                "d" => {
                    let path = args.get(x + 1).map(PathBuf::from);
                    if let Some(path) = path {
                        font_directory = path;
                    } else {
                        missing_arg(bin_name, "-d");
                    }
                    consumed_next = true;
                }
                "f" => {
                    let font = args.get(x + 1);
                    if let Some(font) = font {
                        debug!("font: {font}");
                        if font.ends_with(FIGLET_FONT_SUFFIX) {
                            debug!("about to subtract suffix");
                            font_name = font[0..font.len() - FIGLET_FONT_SUFFIX.len()].to_string();
                        } else {
                            font_name = font.to_string();
                        }
                    } else {
                        missing_arg(bin_name, "-f");
                    }
                    consumed_next = true;
                }
                _ => {
                    error!("Unknown argument: -{arg}");
                    print_usage(bin_name);
                    exit(1);
                }
            }
        } else if let Some(message) = &mut message {
            // figlet adds all arguments to the message
            // unless they start with '-'
            message.push(' ');
            message.push_str(arg);
        } else {
            message = Some(arg.to_string());
        }
    }

    FigletSettings {
        font_name,
        font_directory,
        width,
        justification,
        paragraph,
        right_to_left,
        smush_mode,
        smush_override,
        infocode,
        message,
    }
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
    #[clap(short = 'o', long = "overlap", default_value_t = false, action = clap::ArgAction::SetTrue)]
    overlap: bool,

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

// fn main() {
//     env_logger::builder()
//         .format_timestamp(None)
//         .format_module_path(false)
//         .format_target(false)
//         .init();

//     let args = Args::parse();

//     let settings = match Settings::try_from(args) {
//         Ok(settings) => settings,
//         Err(e) => {
//             error!("Invalid arguments: {e}");
//             std::process::exit(1);
//         }
//     };

//     if settings.info_code >= 0 {
//         utils::print_info(&settings);
//         exit(0);
//     }

//     let font_path = settings
//         .font_directory
//         .join(&settings.font)
//         .with_extension("flf");

//     let font = match FIGfont::from_file(&font_path) {
//         Ok(font) => font,
//         Err(err) => {
//             error!("Failed to load font '{font_path:?}: {err}'");
//             exit(1);
//         }
//     };

//     let mut buf = String::new();
//     if let Err(err) = std::io::stdin().read_to_string(&mut buf) {
//         error!("Failed to read from stdin: {err}");
//         exit(1);
//     }

//     match font.convert(buf, &settings) {
//         Ok(output) => print!("{output}"),
//         Err(err) => {
//             error!("Failed to convert text: {err}");
//             exit(1);
//         }
//     }
// }
