#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
// lints
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
// allowed lints
#![allow(clippy::match_bool)]

mod exact_len;
mod hint_size;
mod invalid_iterator;
mod size_hint;
mod size_hinter;

pub use exact_len::*;
pub use hint_size::*;
pub use invalid_iterator::*;
pub use size_hint::*;
pub use size_hinter::*;
