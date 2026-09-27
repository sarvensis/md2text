//! `md2text` — fast, safe conversion of Markdown into plain text.
//!
//! The crate is built around three goals:
//!
//! * **Fast** — a single streaming pass over the document using the
//!   [`pulldown_cmark`] pull parser; no regexes, no intermediate AST.
//! * **Safe** — 100% safe Rust in the core, the parser never panics on
//!   malformed input, and the public API cannot fail.
//! * **Truncation-tolerant** — half-finished documents (a typical case when
//!   rendering a streaming LLM response token by token) are *healed* before
//!   parsing, so an unterminated code fence still yields clean code text
//!   instead of garbage. See [`Options::heal_truncated`].
//!
//! # Example
//!
//! ```
//! use md2text::{to_plain_text, Options};
//!
//! let md = "# Title\n\nSome **bold** and `code`.";
//! let text = to_plain_text(md, &Options::default());
//! assert_eq!(text, "Title\n\nSome bold and code.");
//! ```

mod convert;
mod heal;
mod options;

pub use options::Options;

/// Convert a Markdown string into plain text.
///
/// This is the single entry point of the library. It is infallible: any input,
/// including empty, malformed, or truncated Markdown, produces a `String`.
///
/// ```
/// use md2text::{to_plain_text, Options};
///
/// let text = to_plain_text("- one\n- two", &Options::default());
/// assert_eq!(text, "- one\n- two");
/// ```
pub fn to_plain_text(markdown: &str, options: &Options) -> String {
    convert::run(markdown, options)
}

#[cfg(feature = "python")]
mod python;
