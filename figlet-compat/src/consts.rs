pub const FIGLET_FONT_SUFFIX: &str = ".flf";
pub const DEFAULT_FONT_FILE: &str = "standard.flf";
pub const DEFAULT_COLUMNS: u16 = 80;

// this ideally would be a u8 but that would break compatibility
pub type Smushmode = i32;

// not a flag, but a value
pub const SM_FULLWIDTH: Smushmode = 0;

// flags
pub const SM_EQUAL: Smushmode = 1 << 0;
pub const SM_LOWLINE: Smushmode = 1 << 1;
pub const SM_HEIRARCHY: Smushmode = 1 << 2;
pub const SM_PAIR: Smushmode = 1 << 3;
pub const SM_BIGX: Smushmode = 1 << 4;
pub const SM_HARDBLANK: Smushmode = 1 << 5;
pub const SM_KERN: Smushmode = 1 << 6;
pub const SM_SMUSH: Smushmode = 1 << 7;

pub type SmushmodeOverride = u8;

pub const SMO_NO: SmushmodeOverride = 0;
pub const SMO_YES: SmushmodeOverride = 1;
pub const SMO_FORCE: SmushmodeOverride = 2;
