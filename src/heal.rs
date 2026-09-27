//! Repair truncated Markdown so it converts cleanly.
//!
//! The conversion pipeline is single-shot: it receives whatever text exists so
//! far (e.g. an LLM response cut off mid-stream) and must produce sensible
//! output. The parser itself never panics on partial input, but an *unclosed*
//! fenced code block is the one construct where a tiny repair noticeably
//! improves the result, so we close it before parsing.

use std::borrow::Cow;

/// Returns the input unchanged unless it ends inside an open fenced code block,
/// in which case a matching closing fence is appended.
///
/// Borrowing is preserved (`Cow::Borrowed`) in the common case where nothing
/// needs to change, so healing is essentially free for complete documents.
pub fn heal(input: &str) -> Cow<'_, str> {
    match open_fence(input) {
        Some(fence_char) => {
            let mut out = String::with_capacity(input.len() + 5);
            out.push_str(input);
            if !input.ends_with('\n') {
                out.push('\n');
            }
            // Three of the same fence character closes the block.
            for _ in 0..3 {
                out.push(fence_char);
            }
            out.push('\n');
            Cow::Owned(out)
        }
        None => Cow::Borrowed(input),
    }
}

/// If the document ends with an unterminated fenced code block, return the
/// fence character (`` ` `` or `~`) that opened it; otherwise `None`.
///
/// A closing fence must use the same character as its opener, so a ```` ``` ````
/// inside a `~~~` block is treated as plain content and does not close it.
fn open_fence(input: &str) -> Option<char> {
    let mut open: Option<char> = None;

    for line in input.lines() {
        let trimmed = line.trim_start();
        let fence = if trimmed.starts_with("```") {
            Some('`')
        } else if trimmed.starts_with("~~~") {
            Some('~')
        } else {
            None
        };

        if let Some(ch) = fence {
            match open {
                None => open = Some(ch),               // opening fence
                Some(active) if active == ch => open = None, // matching close
                Some(_) => {} // different fence char while inside a block: literal
            }
        }
    }

    open
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_input_is_borrowed() {
        let md = "```\ncode\n```\n";
        assert!(matches!(heal(md), Cow::Borrowed(_)));
    }

    #[test]
    fn truncated_fence_is_closed() {
        let healed = heal("```rust\nfn main() {");
        assert!(healed.ends_with("```\n"));
        assert!(healed.contains("fn main() {"));
    }

    #[test]
    fn tilde_fence_closed_with_tilde() {
        let healed = heal("~~~\nfoo");
        assert!(healed.ends_with("~~~\n"));
    }
}
