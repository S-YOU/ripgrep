/*!
ripgrep, as a library.

This library is intended to provide a high level facade to the crates that
make up ripgrep's core searching routines. However, there is no high level
documentation available yet guiding users on how to fit all of the pieces
together.

Every public API item in the constituent crates is documented, but examples
are sparse.

A cookbook and a guide are planned.
*/

#![deny(missing_docs)]

pub use grep_cli as cli;
pub use grep_matcher as matcher;
#[cfg(feature = "pcre2")]
pub extern crate grep_pcre2 as pcre2;
pub use grep_printer as printer;
pub use grep_regex as regex;
pub use grep_searcher as searcher;
