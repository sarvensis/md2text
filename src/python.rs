//! Python bindings (compiled only with the `python` feature).
//!
//! Exposes a stateless `to_text(...)` function and a reusable `Converter`
//! class. The module name (`md2text`) matches the `#[pymodule]` function name
//! and the crate's library name, which is what maturin imports.

use pyo3::prelude::*;

use crate::{to_plain_text, Options};

/// Python spelling of a bool, so `repr()` output is valid Python.
fn py_bool(b: bool) -> &'static str {
    if b {
        "True"
    } else {
        "False"
    }
}

fn default_bullet() -> String {
    "- ".to_string()
}

fn build_options(
    keep_link_urls: bool,
    keep_image_alt: bool,
    list_bullet: Option<String>,
    heal_truncated: bool,
    gfm: bool,
) -> Options {
    Options {
        keep_link_urls,
        keep_image_alt,
        list_bullet: list_bullet.unwrap_or_else(default_bullet),
        heal_truncated,
        gfm,
    }
}

/// Convert a Markdown string to plain text.
///
/// All options are keyword-only so calls stay readable:
/// `md2text.to_text(src, keep_link_urls=True)`.
#[pyfunction]
#[pyo3(signature = (
    markdown,
    *,
    keep_link_urls = false,
    keep_image_alt = true,
    list_bullet = None,
    heal_truncated = true,
    gfm = true,
))]
fn to_text(
    markdown: &str,
    keep_link_urls: bool,
    keep_image_alt: bool,
    list_bullet: Option<String>,
    heal_truncated: bool,
    gfm: bool,
) -> String {
    let options = build_options(
        keep_link_urls,
        keep_image_alt,
        list_bullet,
        heal_truncated,
        gfm,
    );
    to_plain_text(markdown, &options)
}

/// A reusable converter that holds its options.
///
/// Cheaper than passing keyword arguments on every call when you convert many
/// documents with the same settings (e.g. inside a streaming loop).
#[pyclass]
struct Converter {
    options: Options,
}

#[pymethods]
impl Converter {
    #[new]
    #[pyo3(signature = (
        *,
        keep_link_urls = false,
        keep_image_alt = true,
        list_bullet = None,
        heal_truncated = true,
        gfm = true,
    ))]
    fn new(
        keep_link_urls: bool,
        keep_image_alt: bool,
        list_bullet: Option<String>,
        heal_truncated: bool,
        gfm: bool,
    ) -> Self {
        Self {
            options: build_options(
                keep_link_urls,
                keep_image_alt,
                list_bullet,
                heal_truncated,
                gfm,
            ),
        }
    }

    /// Convert a Markdown string using this converter's options.
    fn convert(&self, markdown: &str) -> String {
        to_plain_text(markdown, &self.options)
    }

    fn __repr__(&self) -> String {
        let o = &self.options;
        format!(
            "Converter(keep_link_urls={}, keep_image_alt={}, list_bullet={:?}, \
             heal_truncated={}, gfm={})",
            py_bool(o.keep_link_urls),
            py_bool(o.keep_image_alt),
            o.list_bullet,
            py_bool(o.heal_truncated),
            py_bool(o.gfm),
        )
    }
}

/// Fast, safe Markdown -> plain text conversion (Rust core).
#[pymodule]
fn md2text(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_text, m)?)?;
    m.add_class::<Converter>()?;
    Ok(())
}