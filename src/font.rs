use std::str::FromStr;

pub struct RawHeader {
    pub signature: String,
    pub hardblank: char,
    pub height: u16,
    pub baseline: u16,
    pub max_length: u16,
    pub old_layout: i8,
    pub comment_lines: u16,

    pub direction: Option<u16>,
    pub full_layout: Option<u16>,
    pub codetag_count: Option<u16>,
}

impl FromStr for RawHeader {
    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

pub struct FIGfont {
    header: RawHeader,
}

pub struct FIGUfont {
    pub height: u16,
    pub baseline: u16,
    pub max_length: u16,
}
