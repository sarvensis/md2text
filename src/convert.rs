//! The conversion engine: turn a stream of Markdown events into plain text.
//!
//! We walk [`pulldown_cmark`] events once and append to an output buffer. Block
//! boundaries are managed by a small "separation" helper that guarantees at
//! most one blank line between blocks, so we never accumulate ragged
//! whitespace. Inline markers (emphasis, code-span backticks, link/image
//! syntax) are already stripped by the parser, which is exactly why we lean on
//! it instead of hand-rolling flanking-delimiter rules.

use std::borrow::Cow;

use pulldown_cmark::{Event, Options as MarkdownOptions, Parser, Tag, TagEnd};

use crate::heal;
use crate::options::Options;

/// Entry point used by [`crate::to_plain_text`].
pub(crate) fn run(markdown: &str, opts: &Options) -> String {
    let source: Cow<'_, str> = if opts.heal_truncated {
        heal::heal(markdown)
    } else {
        Cow::Borrowed(markdown)
    };

    let mut ext = MarkdownOptions::empty();
    if opts.gfm {
        ext.insert(MarkdownOptions::ENABLE_TABLES);
        ext.insert(MarkdownOptions::ENABLE_STRIKETHROUGH);
        ext.insert(MarkdownOptions::ENABLE_TASKLISTS);
    }

    let parser = Parser::new_ext(&source, ext);
    let mut writer = Writer::new(opts, source.len());
    for event in parser {
        writer.handle(event);
    }
    writer.finish()
}

/// State for an enclosing list (for nesting indentation and numbering).
struct ListContext {
    ordered: bool,
    index: u64,
}

struct Writer<'a> {
    opts: &'a Options,
    out: String,
    /// Stack of enclosing lists; length == current nesting depth.
    lists: Vec<ListContext>,
    /// Stack of open links; each entry is `Some(url)` if the URL must be
    /// appended when the link closes, else `None`.
    links: Vec<Option<String>>,
    /// When > 0, text events are discarded (used to drop image alt text).
    drop_text: usize,
    /// Table cell capture: while inside a cell, text is buffered separately so
    /// a row can be assembled and emitted as a single line.
    in_cell: bool,
    cell: String,
    row: Vec<String>,
}

impl<'a> Writer<'a> {
    fn new(opts: &'a Options, capacity: usize) -> Self {
        Self {
            opts,
            out: String::with_capacity(capacity),
            lists: Vec::new(),
            links: Vec::new(),
            drop_text: 0,
            in_cell: false,
            cell: String::new(),
            row: Vec::new(),
        }
    }

    /// Append inline text to the active target (table cell or main buffer).
    fn write(&mut self, text: &str) {
        if self.drop_text > 0 {
            return;
        }
        if self.in_cell {
            self.cell.push_str(text);
        } else {
            self.out.push_str(text);
        }
    }

    fn trailing_newlines(&self) -> usize {
        self.out.bytes().rev().take_while(|&b| b == b'\n').count()
    }

    /// Ensure the buffer ends with the newlines needed to start a new block:
    /// one blank line (`blank == true`) or a single line break. No-op at the
    /// start of the document and while capturing a table cell.
    fn separate(&mut self, blank: bool) {
        if self.out.is_empty() || self.in_cell {
            return;
        }
        let want = if blank { 2 } else { 1 };
        for _ in self.trailing_newlines()..want {
            self.out.push('\n');
        }
    }

    fn handle(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.start(tag),
            Event::End(tag) => self.end(tag),
            Event::Text(text) => self.write(&text),
            // Inline/fenced code arrives without backticks.
            Event::Code(code) => self.write(&code),
            // A soft line break inside a paragraph becomes a space (reflow).
            Event::SoftBreak => self.write(" "),
            Event::HardBreak => {
                if self.in_cell {
                    self.cell.push(' ');
                } else {
                    self.out.push('\n');
                }
            }
            Event::Rule => {
                self.separate(true);
                self.out.push_str("---");
            }
            Event::TaskListMarker(done) => {
                self.write(if done { "[x] " } else { "[ ] " });
            }
            // Raw HTML (block and inline) is dropped. Text *between* inline
            // tags still arrives as separate `Text` events, so `<b>x</b>`
            // correctly yields `x`.
            Event::Html(_) | Event::InlineHtml(_) => {}
            // FootnoteReference and any future event variants: ignore.
            _ => {}
        }
    }

    fn start(&mut self, tag: Tag<'_>) {
        match tag {
            // Top-level paragraphs get a blank line; inside a list the text
            // continues on the item's bullet line.
            Tag::Paragraph => {
                if self.lists.is_empty() {
                    self.separate(true);
                }
            }
            Tag::Heading { .. } | Tag::BlockQuote | Tag::CodeBlock(_) => {
                self.separate(true);
            }
            Tag::List(first_number) => {
                self.separate(self.lists.is_empty());
                self.lists.push(ListContext {
                    ordered: first_number.is_some(),
                    index: first_number.unwrap_or(1),
                });
            }
            Tag::Item => {
                self.separate(false);
                let depth = self.lists.len();
                for _ in 1..depth {
                    self.out.push_str("  ");
                }
                let marker = match self.lists.last_mut() {
                    Some(ctx) if ctx.ordered => {
                        let m = format!("{}. ", ctx.index);
                        ctx.index += 1;
                        m
                    }
                    _ => self.opts.list_bullet.clone(),
                };
                self.out.push_str(&marker);
            }
            Tag::Table(_) => self.separate(true),
            Tag::TableHead | Tag::TableRow => {}
            Tag::TableCell => {
                self.in_cell = true;
                self.cell.clear();
            }
            Tag::Link { dest_url, .. } => {
                let url = self
                    .opts
                    .keep_link_urls
                    .then(|| dest_url.to_string());
                self.links.push(url);
            }
            Tag::Image { .. } => {
                if !self.opts.keep_image_alt {
                    self.drop_text += 1;
                }
            }
            // Emphasis / Strong / Strikethrough / FootnoteDefinition /
            // HtmlBlock / MetadataBlock: nothing to do, inner text flows through.
            _ => {}
        }
    }

    fn end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Link => {
                if let Some(Some(url)) = self.links.pop() {
                    self.write(&format!(" ({url})"));
                }
            }
            TagEnd::Image => {
                self.drop_text = self.drop_text.saturating_sub(1);
            }
            TagEnd::TableCell => {
                self.in_cell = false;
                let cell = std::mem::take(&mut self.cell);
                self.row.push(cell.trim().to_string());
            }
            TagEnd::TableHead | TagEnd::TableRow => {
                self.separate(false);
                let line = self.row.join(" | ");
                self.out.push_str(&line);
                self.row.clear();
            }
            TagEnd::List(_) => {
                self.lists.pop();
            }
            // Paragraph, Heading, CodeBlock, BlockQuote, Item, Table,
            // Emphasis, Strong, Strikethrough, etc. need no closing action.
            _ => {}
        }
    }

    fn finish(self) -> String {
        // Block separators never produce more than one blank line, and code
        // content is preserved verbatim, so we only need to trim the document
        // ends.
        self.out.trim().to_string()
    }
}
