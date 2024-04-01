//! # braille-rs
//!
//! A Rust crate that aims to simplify working with braille characters.
//! If you actually want to use braille art in a project i recommend the [rsille](https://crates.io/crates/rsille) crate, since it will most likely be better maintained and has a lot more features.

pub mod braille_char;
pub mod canvas;
pub mod error;

pub use canvas::Canvas;
pub use braille_char::BrailleChar;