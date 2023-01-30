use std::str::FromStr;

use thiserror::Error;

use crate::utils::FONT_FILE_SIGNATURE;

#[derive(Debug)]
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

#[derive(Debug, Error)]
pub enum HeaderParseError {
    #[error("Wrong number of segments: {0}")]
    WrongNumberOfSegments(usize),
    #[error("Invalid signature: {0:?}")]
    InvalidSignature(String),
    #[error("Signature is missing hardblank character")]
    MissingHardblank,
    #[error("Missing segment {1} at index {0}")]
    MissingSegment(usize, &'static str),
    #[error("Failed to parse segment {1} at index {0}: {2:?}")]
    InvalidSegment(usize, &'static str, std::num::ParseIntError),
}

impl FromStr for RawHeader {
    type Err = HeaderParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<_> = s.split(' ').collect();

        if segments.len() < 6 {
            return Err(HeaderParseError::WrongNumberOfSegments(segments.len()));
        }

        let (signature, hardblank) = Self::read_signature(segments[0])?;

        let height = Self::extract_required_segment(&segments, 1, "height")?;
        let baseline = Self::extract_required_segment(&segments, 2, "baseline")?;
        let max_length = Self::extract_required_segment(&segments, 3, "max_length")?;
        let old_layout = Self::extract_required_segment(&segments, 4, "old_layout")?;
        let comment_lines = Self::extract_required_segment(&segments, 5, "comment_lines")?;

        let direction = Self::extract_optional_segment(&segments, 6, "direction")?;
        let full_layout = Self::extract_optional_segment(&segments, 7, "full_layout")?;
        let codetag_count = Self::extract_optional_segment(&segments, 8, "codetag_count")?;

        Ok(Self {
            signature,
            hardblank,
            height,
            baseline,
            max_length,
            old_layout,
            comment_lines,
            direction,
            full_layout,
            codetag_count,
        })
    }
}

impl RawHeader {
    fn read_signature(segment: &str) -> Result<(String, char), HeaderParseError> {
        let signature: String = segment.chars().take(segment.len() - 1).collect();
        if !signature.starts_with(FONT_FILE_SIGNATURE) {
            return Err(HeaderParseError::InvalidSignature(signature));
        }

        let hardblank = segment
            .chars()
            .nth(segment.len() - 1)
            .ok_or(HeaderParseError::MissingHardblank)?;

        Ok((signature, hardblank))
    }

    fn extract_required_segment<T>(
        segments: &[&str],
        idx: usize,
        name: &'static str,
    ) -> Result<T, HeaderParseError>
    where
        T: FromStr<Err = std::num::ParseIntError>,
        T::Err: std::fmt::Debug,
    {
        segments
            .get(idx)
            .ok_or(HeaderParseError::MissingSegment(idx, name))?
            .parse::<T>()
            .map_err(|e| HeaderParseError::InvalidSegment(idx, name, e))
    }

    fn extract_optional_segment<T>(
        segments: &[&str],
        idx: usize,
        name: &'static str,
    ) -> Result<Option<T>, HeaderParseError>
    where
        T: FromStr<Err = std::num::ParseIntError>,
        T::Err: std::fmt::Debug,
    {
        segments
            .get(idx)
            .map(|s| {
                s.parse::<T>()
                    .map_err(|e| HeaderParseError::InvalidSegment(idx, name, e))
            })
            .transpose()
    }
}
