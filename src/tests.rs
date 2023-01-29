use crate::font::RawHeader;

mod small;

#[test]
pub fn test_parse_header() {
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
