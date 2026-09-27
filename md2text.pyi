"""Type stubs for the md2text native extension (Rust / PyO3)."""

from typing import final

__all__ = ["Converter", "to_text"]

def to_text(
    markdown: str,
    *,
    keep_link_urls: bool = False,
    keep_image_alt: bool = True,
    list_bullet: str | None = None,
    heal_truncated: bool = True,
    gfm: bool = True,
) -> str:
    """Convert a Markdown string to plain text.

    All options are keyword-only so calls stay readable:
    ``md2text.to_text(src, keep_link_urls=True)``.

    Args:
        markdown: Markdown source.
        keep_link_urls: Render links as ``text (url)`` instead of just ``text``.
        keep_image_alt: Keep image alt text in the output.
        list_bullet: Marker for unordered list items; ``None`` means ``"- "``.
            Pass ``""`` to drop bullets and keep only the item text.
        heal_truncated: Repair truncated input (e.g. an unterminated code
            fence in a streamed response) before converting.
        gfm: Enable GitHub Flavored Markdown extensions
            (tables, strikethrough, task lists).

    Returns:
        Plain-text rendering of ``markdown``.
    """

@final
class Converter:
    """Reusable converter with fixed options.

    Prefer this over :func:`to_text` in hot loops: options are built once.
    Accepts the same keyword-only options as :func:`to_text`.
    """

    def __new__(
        cls,
        *,
        keep_link_urls: bool = False,
        keep_image_alt: bool = True,
        list_bullet: str | None = None,
        heal_truncated: bool = True,
        gfm: bool = True,
    ) -> Converter: ...
    def convert(self, markdown: str) -> str:
        """Convert a Markdown string using this converter's options."""

    def __repr__(self) -> str: ...
