---
type: Rust Type
title: Element (internal)
description: Crate-private machinery shared by all element builders — tag name, attribute storage, and open/close tag writing.
resource: src/element.rs
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: src
    resource: src/element.rs
    title: htmlforge source — src/element.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, internal, htmlforge]
---

# Element (internal)

`Element<'a>` is `pub(crate)` — it is not part of the public API, but
[`Div`](./div.md) (and any future element builder) is built on top of it.[^src]

## State machine

Each `Element` tracks an `ElementPhase`: `Attributes` or `Children`. It
starts in `Attributes`. Calling `start_children()` writes the opening tag
(with all attributes set so far), increases the writer's indent depth, and
transitions to `Children`. Once in `Children`, `assert_attributes_allowed()`
panics if called — this is what makes setting an attribute after starting a
child a runtime error. See
[attribute/child ordering](../design/attribute-child-ordering.md).

## Attribute storage

Attributes are stored as `Vec<(String, String)>` (insertion order preserved,
not a map):

- `set_attribute` — replaces any existing value for the name.
- `set_attribute_once` — panics if the name is already set (used for `id`).
- `append_attribute` — appends to an existing value with a separator (used
  for `class`).

## Finishing

`finish(self)` calls `start_children()` (so an element with no children
still gets a valid open/close tag pair), dedents, and writes the closing tag.
It only writes a newline/indent before the closing tag if the element
actually had children.

## Text and escaping

`write_text` HTML-escapes the given value via `escape_html` before writing
it. See [HTML escaping](../design/html-escaping.md).

[^src]: htmlforge source — src/element.rs
