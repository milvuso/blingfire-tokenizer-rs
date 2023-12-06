
//! blingfire is a thin Rust wrapper for the
//! [BlingFire](https://github.com/microsoft/BlingFire) tokenization library.

mod errors;

use blingfire_sys::{
    FALimits_MaxArrSize as FA_LIMITS_MAX_ARRAY_SIZE, TextToSentences as text_to_sentences_ffi,
    TextToWords as text_to_words_ffi,
};
use snafu::{self, ensure};
use std::{
    convert::TryInto,
    i32,
    os::raw::{c_char, c_int},
};

pub use crate::errors::{Error, Result};

/// The maximum valid size of the input text for the tokenizer functions.
/// Re-exported from the C++ library.
pub const MAX_TEXT_LENGTH: usize = FA_LIMITS_MAX_ARRAY_SIZE as usize;

/// Tokenizes a piece of text into words separated by whitespace.
///
/// The result of the tokenization operation is stored in the string
/// `destination`. The string will first be cleared.
///
/// ## Errors
///
/// This method returns an error when the input string is too large or when the
/// C++ function fails for an unknown reason (i.e. returns -1).
///
/// ## Example
///
/// ```
/// # fn main() -> Result<(), blingfire::Error> {
///     let mut parsed = String::new();
///     blingfire::text_to_words("Cat,sat on   the mat.", &mut parsed)?;
///     assert_eq!(parsed.as_str(), "Cat , sat on the mat .");
///     # Ok(())
/// # }
/// ```
#[inline]
pub fn text_to_words(source: &str, destination: &mut String) -> Result<()> {
    tokenize(text_to_words_ffi, source, destination)
}