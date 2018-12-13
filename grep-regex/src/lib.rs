/*!
An implementation of `grep-matcher`'s `Matcher` trait for Rust's regex engine.
*/

#![deny(missing_docs)]


#[macro_use]
extern crate log;





pub use crate::error::{Error, ErrorKind};
pub use crate::matcher::{RegexCaptures, RegexMatcher, RegexMatcherBuilder};

mod ast;
mod config;
mod crlf;
mod error;
mod literal;
mod matcher;
mod non_matching;
mod strip;
mod util;
mod word;
