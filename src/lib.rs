//! # braille-rs
//!
//! [<img alt="github" src="https://img.shields.io/badge/github-dtolnay/syn-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mzntori/braille-rs)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/syn.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/braille-rs)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-syn-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/braille-rs)
//!
//! A Rust crate that aims to simplify working with braille characters.
//! If you actually want to use braille art in a project I recommend the [rsille](https://crates.io/crates/rsille) crate, since it will most likely be better maintained and has a lot more features.

pub mod braille_char;
pub mod canvas;
pub mod error;

pub use canvas::Canvas;
pub use braille_char::BrailleChar;