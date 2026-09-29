---
type: Rust Type
title: HtmlWriter (internal)
description: Crate-private output buffer that accumulates the HTML string and tracks indentation for pretty-printing.
resource: src/writer.rs
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: src
    resource: src/writer.rs
    title: htmlforge source — src/writer.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, internal, htmlforge]
---

# HtmlWriter (internal)

`HtmlWriter` is `pub(crate)` — the single shared buffer that every
[`Element`](./element.md) writes into.[^src] `Html` owns one and hands out
`&mut HtmlWriter` as builders are nested.

## Fields

- `output: String` — the accumulated HTML.
- `pretty: bool` — set via `HtmlWriter::new(pretty)`; `Html::new()` passes
  `false`, `Html::pretty()` passes `true`.
- `depth: usize` — current indentation depth, adjusted by `indent()` /
  `dedent()` as elements are entered and exited.

## Pretty-printing

`begin_child()` is called before writing any element's open tag or any text
node. In pretty mode, and only once the output is non-empty (so the very
first node isn't prefixed with a blank line), it writes a newline followed
by two spaces per unit of `depth`. In non-pretty mode it's a no-op. See
[pretty-printing](../design/pretty-printing.md).

[^src]: htmlforge source — src/writer.rs
