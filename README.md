# md2text

A fast, safe Rust library for converting Markdown to plain text, resilient to
truncated input, with Python bindings.

## Design rationale

Parsing is handled by `pulldown-cmark`, a battle-tested CommonMark parser
(pull-based, single pass, panic-free). On top of it sits a custom
event-to-text converter. This is a deliberate choice: a naive stripper that
removes `*`/`_` markers breaks `snake_case`, arithmetic like `2 * 3`, file
paths, and so on. A real parser correctly applies the flanking-delimiter
rules, so such text is left intact.

## Features

- **Fast** — a single streaming pass, no regex, no AST construction.
- **Safe** — the core is 100% safe Rust, and the API never returns errors:
  any input produces a `String`.
- **Truncated input** — an unclosed ```` ``` ```` block is "healed" before
  parsing, so a cut-off stream (e.g. an LLM response arriving token by token)
  yields clean code instead of garbage.
- **Configurable** — links with or without URLs, image alt text, list
  markers, GFM (tables, strikethrough, task lists).

## Building

Rust crate:

```bash
cargo build --release
cargo test            # unit and integration tests
cargo run --example basic
```

Python package (via maturin):

```bash
pip install maturin
maturin develop --release        # build and install into the current venv
python python/test_md2text.py    # sanity check
# or build a wheel:
maturin build --release
```

## Usage (Rust)

```rust
use md2text::{to_plain_text, Options};

let md = "# Heading\n\nText with **bold** and `code`.";
let text = to_plain_text(md, &Options::default());
assert_eq!(text, "Heading\n\nText with bold and code.");

// Keep link URLs:
let opts = Options { keep_link_urls: true, ..Options::default() };
let text = to_plain_text("[site](https://example.com)", &opts);
assert_eq!(text, "site (https://example.com)");
```

## Usage (Python)

```python
import md2text

md2text.to_text("# Hi\n\n**bold** and `code`")
# -> "Hi\n\nbold and code"

md2text.to_text("[x](https://example.com)", keep_link_urls=True)
# -> "x (https://example.com)"

# Reusable converter for hot loops:
conv = md2text.Converter(keep_link_urls=True, gfm=True)
conv.convert(chunk)
```

## Options

| Field / kwarg    | Default | Description                                               |
|------------------|---------|-----------------------------------------------------------|
| `keep_link_urls` | `false` | Append the URL after the link text: `text (url)`.         |
| `keep_image_alt` | `true`  | Keep image alt text (otherwise the image is dropped).     |
| `list_bullet`    | `"- "`  | Marker for unordered list items; `""` for no marker.      |
| `heal_truncated` | `true`  | Close unterminated code blocks before parsing.            |
| `gfm`            | `true`  | Tables, strikethrough, task lists.                        |

## Behavior on truncated input

`heal_truncated` closes an unterminated ```` ``` ```` block, so code from a
cut-off stream is preserved as text. Unclosed **inline** constructs (`**bold`,
an unterminated code span) are emitted by the parser, per CommonMark rules, as
plain text including the marker characters themselves — this is expected and
does not cause a failure. The core guarantee: on any partial input the
function never panics and returns meaningful text.

## License

Apache-2.0

---

Быстрая и безопасная библиотека на Rust для превращения Markdown в обычный текст,
устойчивая к обрезанному вводу, с биндингами для Python.

## Почему так устроено

Ядро разбора — проверенный CommonMark-парсер `pulldown-cmark` (pull-парсер,
один проход, без паник). Поверх него — собственный конвертер событий в текст.
Это сознательный выбор: наивный стриппер маркеров `*`/`_` ломает `snake_case`,
арифметику `2 * 3`, пути и т.п. Парсер же корректно применяет правила
flanking-делимитеров, поэтому такой текст остаётся нетронутым.

## Возможности

- **Быстро** — один потоковый проход, без regex и без построения AST.
- **Безопасно** — только safe Rust в ядре, API не возвращает ошибок: любой ввод
  даёт `String`.
- **Обрезанный ввод** — незакрытый блок ``` ``` ``` «залечивается» перед
  разбором, поэтому оборванный поток (например, ответ LLM по токенам) даёт
  чистый код, а не мусор.
- **Конфигурируемо** — ссылки с URL или без, alt у картинок, маркеры списков,
  GFM (таблицы, зачёркивание, чек-листы).

## Сборка

Rust-крейт:

```bash
cargo build --release
cargo test            # юнит- и интеграционные тесты
cargo run --example basic
```

Python-пакет (через maturin):

```bash
pip install maturin
maturin develop --release        # собрать и поставить в текущее venv
python python/test_md2text.py    # проверка
# либо собрать колесо:
maturin build --release
```

## Использование (Rust)

```rust
use md2text::{to_plain_text, Options};

let md = "# Заголовок\n\nТекст с **жирным** и `кодом`.";
let text = to_plain_text(md, &Options::default());
assert_eq!(text, "Заголовок\n\nТекст с жирным и кодом.");

// С URL у ссылок:
let opts = Options { keep_link_urls: true, ..Options::default() };
let text = to_plain_text("[сайт](https://example.com)", &opts);
assert_eq!(text, "сайт (https://example.com)");
```

## Использование (Python)

```python
import md2text

md2text.to_text("# Hi\n\n**bold** and `code`")
# -> "Hi\n\nbold and code"

md2text.to_text("[x](https://example.com)", keep_link_urls=True)
# -> "x (https://example.com)"

# Переиспользуемый конвертер для горячего цикла:
conv = md2text.Converter(keep_link_urls=True, gfm=True)
conv.convert(chunk)
```

## Опции

| Поле / kwarg     | По умолчанию | Описание                                                  |
|------------------|--------------|-----------------------------------------------------------|
| `keep_link_urls` | `false`      | Дописывать URL после текста ссылки: `текст (url)`.        |
| `keep_image_alt` | `true`       | Сохранять alt-текст картинок (иначе картинка отбрасывается). |
| `list_bullet`    | `"- "`       | Маркер ненумерованного списка; `""` — без маркера.        |
| `heal_truncated` | `true`       | Чинить незакрытые блоки кода перед разбором.              |
| `gfm`            | `true`       | Таблицы, зачёркивание, чек-листы.                         |

## Поведение на обрезанном вводе

`heal_truncated` закрывает незавершённый блок ``` ``` ```, поэтому код из
оборванного потока сохраняется как текст. Незакрытые **строчные** конструкции
(`**жир`, незавершённый код-спан) парсер по правилам CommonMark выводит как
обычный текст с самими символами-маркерами — это ожидаемо и не приводит к сбою.
Главная гарантия: на любом частичном вводе функция не паникует и возвращает
осмысленный текст.

## Лицензия

Apache-2.0
