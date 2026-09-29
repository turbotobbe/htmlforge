---
type: Design Note
title: HTML escaping
description: How text nodes and attribute values are escaped to prevent malformed or unsafe output.
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: element
    resource: src/element.rs
    title: htmlforge source — src/element.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, design, htmlforge, security]
---

# HTML escaping

`escape_html`, a free function in `src/element.rs`, escapes five characters:
`&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;`, `"` → `&quot;`, `'` → `&#39;`.[^element]

It is applied in two places:

- `Element::write_open_tag` escapes every attribute value before writing it
  inside the `"..."` delimiters.
- `Element::write_text` escapes the value passed to `Div::text` before
  writing it as a text node.

Both call sites go through the shared `Element`, so escaping is not
something individual element builders (like [`Div`](../api/div.md)) opt
into — it's applied uniformly to all attribute values and text content
written through `htmlforge`.

[^element]: htmlforge source — src/element.rs
