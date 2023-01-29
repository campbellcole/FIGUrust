use crate::settings::Settings;

pub const FONT_FILE_SUFFIX: &str = ".frf";
pub const FONT_FILE_MAGIC_NUMBER: u32 = to_magic_number(['F', 'R', 'F', '0']);

const fn to_magic_number(chars: [char; 4]) -> u32 {
    (chars[0] as u32) << (3 * 8)
        | (chars[1] as u32) << (2 * 8)
        | (chars[2] as u32) << 8
        | chars[3] as u32
}

pub fn print_info(settings: &Settings) {
    match settings.info_code {
        0 => {
            // copyright message
            println!("FIGUrust Copyright (C) 2023 Campbell Cole\n");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Build date: {}", env!("BUILD_DATE"));
            println!("Git hash: {}\n", env!("GIT_HASH"));
            println!("FIGUrust, along with the various FIGUrust fonts and documentation,");
            println!("may be freely copied, modified, and redistributed.\n");
            println!("The latest version of FIGUrust is available at:");
            println!("https://github.com/campbellcole/FIGUrust\n");
            println!("This project is a Rust implementation of FIGlet:");
            println!("http://www.figlet.org/")
        }
        1 => {
            // version message
            println!("FIGUrust {}", env!("CARGO_PKG_VERSION"));
        }
        2 => {
            // font directory
            println!("{}", settings.font_directory.display());
        }
        3 => {
            // font name
            println!("{}", settings.font);
        }
        4 => {
            // font width
            println!("{}", settings.width());
        }
        5 => {
            // font format magic number
            println!("{}", FONT_FILE_MAGIC_NUMBER);
        }
        _ => {}
    }
}
