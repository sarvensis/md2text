"""Quick usage check. Run after `maturin develop`:  python python/test_md2text.py"""

import md2text

# Stateless function with keyword options.
print(md2text.to_text("# Hi\n\n**bold** and `code`"))
# -> "Hi\n\nbold and code"

print(md2text.to_text("[x](https://example.com)", keep_link_urls=True))
# -> "x (https://example.com)"

# Truncated stream: an unterminated code fence is healed automatically.
partial = "Here:\n\n```python\ndef f():\n    return 1"
print(md2text.to_text(partial))

# Reusable converter for hot loops.
conv = md2text.Converter(keep_link_urls=True, gfm=True)
print(repr(conv))
print(conv.convert("See [docs](https://docs.example.com) for ~~old~~ new info."))


def test_basic():
    assert md2text.to_text("**x**") == "x"
    assert md2text.to_text("snake_case 2 * 3") == "snake_case 2 * 3"
    assert "def f():" in md2text.to_text(partial)


if __name__ == "__main__":
    test_basic()
    print("\nOK")
