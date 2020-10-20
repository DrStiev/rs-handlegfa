/// file that is used to define all the common types that can be 
/// parsed and used as SegmentId
use crate::parser_gfa2::ParseFieldError;

use bstr::{BString, ByteSlice};
use lazy_static::lazy_static;
use regex::bytes::Regex;

// FIXME SENPAI
/*
impl<S: SegmentId> std::fmt::Display for dyn S 
where
    S: SegmentId,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
*/

/// Trait for the types that can be parsed and used as segment IDs;
/// will probably only be usize and BString.
pub trait SegmentId: Sized + Default {
    const ERROR: ParseFieldError;

    // define the functions
    fn parse_opt_id(input: &[u8]) -> Option<Self>;
    fn parse_id(input: &[u8]) -> Option<Self>;
    fn parse_ref(input: &[u8]) -> Option<Self>;

    fn parse_next<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_id(next.as_ref()).ok_or(Self::ERROR)
    }

    fn parse_next_opt<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_opt_id(next.as_ref()).ok_or(Self::ERROR)
    }

    fn parse_next_ref<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_ref(next.as_ref()).ok_or(Self::ERROR)
    }
}

impl SegmentId for usize {
    const ERROR: ParseFieldError = ParseFieldError::UintIdError;

    fn parse_id(input: &[u8]) -> Option<Self> {
        input.to_str().ok()?.parse::<usize>().ok()
    }

    fn parse_opt_id(input: &[u8]) -> Option<Self> {
        input.to_str().ok()?.parse::<usize>().ok()
    }

    fn parse_ref(input: &[u8]) -> Option<Self> {
        input.to_str().ok()?.parse::<usize>().ok()
    }
}

impl SegmentId for BString {
    const ERROR: ParseFieldError = ParseFieldError::Utf8Error;

    fn parse_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?-u)[!-~]+").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }

    fn parse_opt_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?-u)[!-~]+|\*").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }

    fn parse_ref(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?-u)[!-~]+[+-]").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }
}