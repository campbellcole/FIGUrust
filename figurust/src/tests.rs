use std::sync::Once;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn, LevelFilter};

use crate::font::{FIGfont, RawHeader};

mod small;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        env_logger::builder()
            .filter_level(LevelFilter::Trace)
            .is_test(false)
            .init();
    });
}

#[test]
pub fn test_parse_header() {
    init();

    use small::*;
    let small_font = include_str!("tests/small.flf");
    let header_line = small_font
        .lines()
        .next()
        .expect("small.flf has no first line");

    let header = header_line
        .parse::<RawHeader>()
        .expect("failed to parse header");

    assert_eq!(header.signature, SMALL_SIGNATURE);
    assert_eq!(header.hardblank, SMALL_HARDBLANK);

    assert_eq!(header.height, SMALL_HEIGHT);
    assert_eq!(header.baseline, SMALL_BASELINE);
    assert_eq!(header.max_length, SMALL_MAX_LENGTH);
    assert_eq!(header.old_layout, SMALL_OLD_LAYOUT);
    assert_eq!(header.comment_lines, SMALL_COMMENT_LINES);

    assert_eq!(header.direction, SMALL_DIRECTION);
    assert_eq!(header.full_layout, SMALL_FULL_LAYOUT);
    assert_eq!(header.codetag_count, SMALL_CODETAG_COUNT);
}

#[test]
pub fn test_parse_characters() {
    init();

    let small_font = include_str!("tests/small.flf");
    let _ = small_font.parse::<FIGfont>().expect("failed to parse font");
}

#[test]
pub fn test_convert_text() {
    init();

    let small_font = include_str!("tests/small.flf");
    let font = small_font.parse::<FIGfont>().expect("failed to parse font");

    let text = "Hello, world!";
    let converted = font.convert(text).expect("failed to convert text");

    let expected = r#"
  _  _         _   _                                    _      _   _ 
 | || |  ___  | | | |  ___        __ __ __  ___   _ _  | |  __| | | |
 | __ | / -_) | | | | / _ \  _    \ V  V / / _ \ | '_| | | / _` | |_|
 |_||_| \___| |_| |_| \___/ ( )    \_/\_/  \___/ |_|   |_| \__,_| (_)
                            |/                                       
"#;

    assert_eq!(converted, expected[1..]);
}
