---
type: Design Note
title: Pretty-printing
description: How Html::pretty() vs Html::new() controls indentation and line breaks in the output.
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: writer
    resource: src/writer.rs
    title: htmlforge source — src/writer.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
  - id: lib
    resource: src/lib.rs
    title: htmlforge source — src/lib.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, design, htmlforge]
---

# Pretty-printing

`Html::new()` produces compact, single-line output. `Html::pretty()`
produces indented, multi-line output — each element and text node on its own
line.[^lib] The behavior is controlled by a single `pretty: bool` flag stored
on [`HtmlWriter`](../api/writer.md), set once at construction time and never
changed afterward.[^writer]

## Mechanism

- `HtmlWriter::indent()` / `dedent()` adjust a `depth: usize` counter as
  elements are entered (`Element::start_children`) and exited
  (`Element::finish`).
- `HtmlWriter::begin_child()` is called before every open tag and every text
  node. In pretty mode it emits `\n` followed by `"  ".repeat(depth)`; in
  compact mode it does nothing.
- The very first node written is not prefixed with a newline, since
  `begin_child()` only acts when the output is already non-empty.

There is no way to change indentation width (fixed at two spaces) or to
toggle pretty mode after construction — it's a one-time choice per `Html`
instance.

[^lib]: htmlforge source — src/lib.rs
[^writer]: htmlforge source — src/writer.rs
