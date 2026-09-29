---
type: Design Note
title: Closure-based builder pattern
description: htmlforge nests element builders via closures (FnOnce(&mut Div)) rather than a fluent method chain.
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: lib
    resource: src/lib.rs
    title: htmlforge source — src/lib.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
  - id: div
    resource: src/div.rs
    title: htmlforge source — src/div.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, design, htmlforge]
---

# Closure-based builder pattern

Both [`Html::div`](../api/html.md) and [`Div::div`](../api/div.md) take a
closure — `F: FnOnce(&mut Div<'_>)` — rather than returning a builder for the
caller to chain against or drive manually:[^lib][^div]

```rust
html.div(|div| {
    div.id("foo");
    div.text("Hello");
});
```

The parent method constructs the child builder, invokes the closure with a
`&mut` reference to it, and then calls `finish()` on the child itself — the
caller never sees or holds the child builder outside the closure's scope.
This structurally guarantees a child element is always finished (its closing
tag written) before its parent continues, without relying on `Drop` or the
caller remembering to call a method.

## Related

- [Attribute/child ordering](./attribute-child-ordering.md) is enforced
  within a single builder's lifetime, using the same nesting discipline.

[^lib]: htmlforge source — src/lib.rs
[^div]: htmlforge source — src/div.rs
