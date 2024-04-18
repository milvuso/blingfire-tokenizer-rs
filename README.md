
[![Build Status](https://travis-ci.com/milvuso/blingfire-tokenizer-rs.svg?branch=master)](https://travis-ci.com/milvuso/blingfire-tokenizer-rs)
[![Documentation](https://docs.rs/blingfire/badge.svg)](https://docs.rs/blingfire)
[![Crate](https://meritbadge.herokuapp.com/blingfire)](https://crates.io/crates/blingfire)

# BlingFire Tokenizer in Rust

`blingfire` is a convenient Rust wrapper for the [BlingFire](https://github.com/microsoft/BlingFire) tokenization library.

Add the library to your project's `Cargo.toml` file to start using it
```bash
cargo add blingfire
```

The library furnishes two key functions `text_to_words` and `text_to_sentences`
```rust
use blingfire;

fn main() {
    let mut parsed = String::new();

    blingfire::text_to_words("Cat,sat on   the mat.", &mut parsed).unwrap();
    assert_eq!(parsed.as_str(), "Cat , sat on the mat .");

    blingfire::text_to_sentences("Cat sat. Dog barked.", &mut parsed).unwrap();
    assert_eq!(parsed.as_str(), "Cat sat.\nDog barked.");
}
```

This project is licensed under MIT License.