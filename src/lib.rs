#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]

mod exact_len;
mod hint_size;
mod size_hinter;

pub use exact_len::*;
pub use hint_size::*;
pub use size_hinter::*;
