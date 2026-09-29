---
type: Design Note
title: Attributes must precede children
description: Setting an attribute after a child has been started panics — attribute methods and start_children() share an ElementPhase state machine.
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: element
    resource: src/element.rs
    title: htmlforge source — src/element.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
  - id: attributes
    resource: src/attributes.rs
    title: htmlforge source — src/attributes.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, design, htmlforge]
---

# Attributes must precede children

Every [`Element`](../api/element.md) tracks an `ElementPhase` —
`Attributes` or `Children` — starting in `Attributes`. Writing the opening
tag (which requires knowing the final attribute set up front, since
attributes appear inside `<tag ...>`) happens lazily, the first time a child
is started via `start_children()`.[^element]

Every method in the [`Attributes`](../api/attributes.md) trait (`id`,
`class`, `title`, `attribute`) calls `assert_attributes_allowed()` first,
which panics with `"HTML attributes must be declared before children"` if
the element has already transitioned to `Children`.[^attributes] For
example, this panics:

```rust
div.text("Hello");
div.class("too-late"); // panics: attributes already closed off
```

This is a runtime check, not a compile-time one — the type system doesn't
prevent calling an attribute setter after `text()` or a nested `div()` call,
so the constraint is enforced via `assert!`/`assert_eq!` inside the shared
`Element`.

[^element]: htmlforge source — src/element.rs
[^attributes]: htmlforge source — src/attributes.rs
