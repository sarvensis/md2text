use md2text::{to_plain_text, Options};

fn convert(md: &str) -> String {
    to_plain_text(md, &Options::default())
}

#[test]
fn strips_inline_formatting() {
    let md = "# Title\n\nSome **bold** and *italic* and `code`.";
    assert_eq!(convert(md), "Title\n\nSome bold and italic and code.");
}

#[test]
fn does_not_mangle_code_like_text() {
    // The classic failure mode of naive strippers.
    let md = "Use `snake_case_name` and compute 2 * 3 * 4.";
    assert_eq!(convert(md), "Use snake_case_name and compute 2 * 3 * 4.");
}

#[test]
fn links_drop_url_by_default() {
    assert_eq!(convert("[Google](https://google.com)"), "Google");
}

#[test]
fn links_keep_url_when_requested() {
    let opts = Options { keep_link_urls: true, ..Options::default() };
    assert_eq!(
        to_plain_text("[Google](https://google.com)", &opts),
        "Google (https://google.com)"
    );
}

#[test]
fn fenced_code_is_preserved_without_fences() {
    let md = "```rust\nfn main() {}\n```";
    assert_eq!(convert(md), "fn main() {}");
}

#[test]
fn truncated_code_fence_is_healed() {
    // Simulates a stream cut off mid code block.
    let md = "Here:\n\n```rust\nfn main() {\n    let x = 1;";
    let out = convert(md);
    assert!(out.contains("fn main() {"));
    assert!(out.contains("let x = 1;"));
    assert!(!out.contains("```"));
}

#[test]
fn unordered_list() {
    assert_eq!(convert("- one\n- two\n- three"), "- one\n- two\n- three");
}

#[test]
fn ordered_list_numbers() {
    assert_eq!(convert("1. a\n2. b"), "1. a\n2. b");
}

#[test]
fn nested_list_is_indented() {
    assert_eq!(convert("- a\n  - b"), "- a\n  - b");
}

#[test]
fn blockquote_marker_removed() {
    assert_eq!(convert("> quoted text"), "quoted text");
}

#[test]
fn image_alt_kept_by_default() {
    assert_eq!(convert("![a cat](cat.png)"), "a cat");
}

#[test]
fn image_dropped_when_alt_disabled() {
    let opts = Options { keep_image_alt: false, ..Options::default() };
    assert_eq!(to_plain_text("![a cat](cat.png)", &opts), "");
}

#[test]
fn gfm_strikethrough_and_table() {
    assert_eq!(convert("~~gone~~"), "gone");

    let table = "| A | B |\n|---|---|\n| 1 | 2 |";
    assert_eq!(convert(table), "A | B\n1 | 2");
}

#[test]
fn inline_html_tags_dropped_but_text_kept() {
    assert_eq!(convert("a <b>bold</b> word"), "a bold word");
}

#[test]
fn empty_input_is_empty() {
    assert_eq!(convert(""), "");
    assert_eq!(convert("   \n\n  "), "");
}
