---
type: Rust API
title: Attributes
description: Trait providing id/class/title/attribute setters to element builders.
resource: src/attributes.rs
status: draft
generated: { by: claude-code/sonnet-5, at: 2026-09-29T22:59:39Z }
sources:
  - id: src
    resource: src/attributes.rs
    title: htmlforge source — src/attributes.rs
    author: human:thobias.bergqvist
    last_modified: 2026-09-30T00:50:45+02:00
tags: [rust, api, htmlforge]
---

# Attributes

`Attributes<'a>` is a trait implemented by element builders (currently just
[`Div`](./div.md)) to provide a common set of attribute-setting methods. It
requires one method, `element(&mut self) -> &mut Element<'a>`, and supplies
default implementations on top of it.[^src]

## Methods

- `id(value: impl Display)` — sets the `id` attribute. Delegates to
  `Element::set_attribute_once`, so setting `id` twice on the same element
  panics.
- `class(value: impl Display)` — sets the `class` attribute. Delegates to
  `Element::append_attribute` with a space separator, so repeated calls
  accumulate classes (e.g. `class("boo"); class("far")` produces
  `class="boo far"`).
- `title(value: impl Display)` — sets the `title` attribute via `attribute`.
- `attribute(name: &str, value: impl Display)` — sets an arbitrary attribute
  by name, replacing any previous value under that name.

All methods call `Element::assert_attributes_allowed`, which panics if any
child (element or text) has already been started on that element. See
[attribute/child ordering](../design/attribute-child-ordering.md).

[^src]: htmlforge source — src/attributes.rs
