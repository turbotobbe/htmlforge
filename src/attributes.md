# `src/attributes.rs` — a trait with default methods

This file defines `Attributes`: a trait shared by element builders (right
now, just `Div`) that adds `.id(...)`, `.class(...)`, `.title(...)`, and
`.attribute(...)` methods. It's a compact but rich example of what traits
are *for* in Rust, beyond the simple "implement `Default`" cases seen in
[`src/writer.md`](writer.md) and [`src/lib.md`](lib.md).

```rust
use std::fmt::Display;

use crate::element::Element;

pub trait Attributes<'a> {
    fn element(&mut self) -> &mut Element<'a>;

    fn id(&mut self, value: impl Display) {
        let element = self.element();
        element.assert_attributes_allowed();
        element.set_attribute_once("id", value);
    }

    fn class(&mut self, value: impl Display) {
        let element = self.element();
        element.assert_attributes_allowed();
        element.append_attribute("class", value, " ");
    }

    fn title(&mut self, value: impl Display) {
        self.attribute("title", value);
    }

    fn attribute(&mut self, name: &str, value: impl Display) {
        let element = self.element();
        element.assert_attributes_allowed();
        element.set_attribute(name, value);
    }
}
```

## What a trait is

A `trait` is a contract: a named set of methods that a type can promise to
provide. You've already seen the *consuming* side of this — implementing
the standard library's `Default` trait in the two previous files. Here,
`Attributes` is a trait this crate defines itself, for its own purposes.

## Required vs. default methods

```rust
pub trait Attributes<'a> {
    fn element(&mut self) -> &mut Element<'a>;

    fn id(&mut self, value: impl Display) {
        ...
    }
    ...
}
```

Look closely at the difference between the first method and the rest:

- `fn element(&mut self) -> &mut Element<'a>;` ends in a semicolon — it has
  **no body**. This is a *required* method: any type implementing
  `Attributes` must provide its own implementation. (You saw `Div` do
  exactly that in [`src/div.md`](div.md).)
- `fn id(...) { ... }`, `fn class(...) { ... }`, `fn title(...) { ... }`,
  and `fn attribute(...) { ... }` all have bodies. These are **default
  methods**: the trait itself supplies an implementation, and any
  implementing type gets them for free, automatically, without writing a
  single line of code for them.

This is the core trick of the file: by implementing *one* small method
(`element`), `Div` gets *four* fully-working methods handed to it. If a
second element builder existed in the future (say, a `Span`), it would only
need to implement `element` too, and it would immediately gain identical
`id`/`class`/`title`/`attribute` behavior — no copy-pasting required. This
is why it's a trait and not just methods written directly on `Div`: the
behavior is meant to be *shared* across every element builder, while each
builder only has to supply the one thing that's actually specific to it
(how to get at its own `Element`).

## Default methods calling the required method

```rust
fn id(&mut self, value: impl Display) {
    let element = self.element();
    element.assert_attributes_allowed();
    element.set_attribute_once("id", value);
}
```

Every default method starts by calling `self.element()` — the required
method — to get at the underlying `Element`. This is the whole point of
having a required method in the first place: the trait's default methods
are generic logic written once, on top of one small hook that each
implementing type fills in differently. `Div::element` happens to return
`&mut self.element`, but the trait code here doesn't know or care about
that; it only knows it can call `self.element()` and get *some*
`&mut Element<'a>` back.

From there, each method delegates to `Element`'s own methods (see
[`src/element.md`](element.md) for what they actually do):

- `id` calls `set_attribute_once("id", value)` — setting `id` a second time
  panics.
- `class` calls `append_attribute("class", value, " ")` — repeated calls
  accumulate, space-separated, instead of overwriting.
- `attribute` calls the plain `set_attribute(name, value)` — repeated calls
  with the same name overwrite.
- `title` doesn't touch `Element` directly at all.

## One default method calling another

```rust
fn title(&mut self, value: impl Display) {
    self.attribute("title", value);
}
```

`title` is implemented purely in terms of another default method on the
*same* trait, `attribute`. This works because inside a trait's default
method, `self` is understood to be "whatever type is implementing this
trait," and that type is guaranteed (by the trait definition) to have
*all* of `Attributes`'s methods available on it — including the other
default ones. So `title` can call `self.attribute(...)` exactly as if it
were calling a regular method, even though, from `title`'s point of view,
it doesn't know what concrete type `Self` actually is.

## `impl Display` again

```rust
fn id(&mut self, value: impl Display) {
```

Same "impl Trait in argument position" pattern introduced in
[`src/div.md`](div.md): `value` can be anything printable — a `&str`, a
number, or any custom type implementing `Display`.

## Why the trait itself needs a lifetime: `Attributes<'a>`

```rust
pub trait Attributes<'a> {
    fn element(&mut self) -> &mut Element<'a>;
    ...
}
```

The required method's return type, `&mut Element<'a>`, mentions a lifetime.
For that to type-check, `'a` has to be introduced *somewhere* — and since
it's not tied to any particular method's own parameters, it's declared on
the trait itself, in `Attributes<'a>`. Every method in the trait — even the
default ones that never mention `Element` directly, like `title` — is then
implicitly parameterized by that same `'a`. This is exactly the lifetime
that gets threaded through in [`src/div.md`](div.md)'s
`impl<'a> Attributes<'a> for Div<'a>`.

## A practical gotcha: trait methods need the trait in scope

Because `id`, `class`, `title`, and `attribute` are *trait* methods rather
than methods defined directly on `Div`, Rust only lets you call them on a
`Div` value if `Attributes` is imported into scope with `use` at the call
site — which is exactly why every example in this crate (and this crate's
own [`README.md`](../README.md)) starts with:

```rust
use htmlforge::{Attributes, Html};
```

Forget the `Attributes` half of that import, and `div.id("foo")` fails to
compile with an error like *"no method named `id` found for mutable
reference to `Div` in the current scope"* — even though the method clearly
exists. This is a deliberate Rust design choice (it keeps you from
accidentally picking up methods from traits you didn't know were
implemented for a type), but it does trip up newcomers, so it's worth
remembering.

## See also

- [`src/div.md`](div.md) — the one concrete type implementing `Attributes`
  today, and what its required `element()` method looks like.
- [`src/element.md`](element.md) — `assert_attributes_allowed`,
  `set_attribute_once`, `set_attribute`, and `append_attribute`: what these
  default methods actually delegate to.
