---
type: Rust API
title: Html
description: Root builder for constructing an HTML document with htmlforge.
resource: src/lib.rs
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: src
    resource: src/lib.rs
    title: htmlforge source — src/lib.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, api, htmlforge]
---

# Html

`Html` is the entry point for building a document. It wraps an internal
[`HtmlWriter`](./writer.md) that accumulates the output string.[^src]

## Construction

- `Html::new()` — compact output, no indentation or line breaks.
- `Html::pretty()` — indents nested elements and breaks each element or text
  node onto its own line. See [pretty-printing](../design/pretty-printing.md).
- `Html` also implements `Default`, equivalent to `Html::new()`.

## Building content

- `html.div(|div| { ... })` — opens a [`Div`](./div.md) builder, passing it by
  mutable reference to the given closure. See
  [the builder pattern](../design/builder-pattern.md).

## Finishing

- `html.finish() -> String` — consumes the builder and returns the
  accumulated HTML string.

# Examples

```rust
use htmlforge::{Attributes, Html};

let mut html = Html::pretty();
html.div(|div| {
    div.id("foo");
    div.class("bar");
    div.text("Hello");
});

let output = html.finish();
```

[^src]: htmlforge source — src/lib.rs
