---
type: Rust API
title: Div
description: Builder for a <div> element, its attributes, text, and nested divs.
resource: src/div.rs
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: src
    resource: src/div.rs
    title: htmlforge source — src/div.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, api, htmlforge]
---

# Div

`Div<'a>` builds a single `<div>` element. It wraps an
[`Element<'a>`](./element.md) configured with the tag name `"div"`, and
implements the [`Attributes`](./attributes.md) trait for `id`, `class`,
`title`, and arbitrary attributes.[^src]

Instances are only constructed internally (`pub(crate) fn new`) — callers
obtain one via [`Html::div`](./html.md) or `Div::div` (for nesting), never
directly.

## Methods

- `div.text(value: impl Display)` — writes a text node as a child,
  HTML-escaped. See [HTML escaping](../design/html-escaping.md).
- `div.div(|child| { ... })` — opens a nested `Div` builder for a child
  `<div>`, following the same [builder pattern](../design/builder-pattern.md)
  as `Html::div`.
- `div.finish(self)` — consumes the builder and writes the closing tag.
  Called automatically by `Html::div` and the parent `Div::div` after the
  closure returns.

## Related

- Attribute methods (`id`, `class`, `title`, `attribute`) come from the
  [`Attributes`](./attributes.md) trait.
- Calling an attribute method after a child has been started panics — see
  [attribute/child ordering](../design/attribute-child-ordering.md).

[^src]: htmlforge source — src/div.rs
