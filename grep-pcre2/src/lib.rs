/*!
An implementation of `grep-matcher`'s `Matcher` trait for
[PCRE2](https://www.pcre.org/).
*/

#![deny(missing_docs)]




pub use crate::error::{Error, ErrorKind};
pub use crate::matcher::{RegexCaptures, RegexMatcher, RegexMatcherBuilder};

mod error;
mod matcher;
