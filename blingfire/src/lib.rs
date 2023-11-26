
//! blingfire is a thin Rust wrapper for the
//! [BlingFire](https://github.com/microsoft/BlingFire) tokenization library.

mod errors;

use blingfire_sys::{
    FALimits_MaxArrSize as FA_LIMITS_MAX_ARRAY_SIZE, TextToSentences as text_to_sentences_ffi,
    TextToWords as text_to_words_ffi,
};