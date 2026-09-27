use md2text::{to_plain_text, Options};

fn main() {
    let md = "\
# Release notes

Some **bold** text, an *italic* word and a [link](https://example.com).

```rust
fn main() {
    println!(\"hello\");
}
```

- first item
- second item
  - nested item
";

    println!("== default ==\n{}\n", to_plain_text(md, &Options::default()));

    let with_urls = Options { keep_link_urls: true, ..Options::default() };
    println!("== keep_link_urls ==\n{}", to_plain_text(md, &with_urls));
}
