# `src/div.rs` — the `<div>` element builder

`Div` is the first (and currently only) concrete element builder in the
crate. It's a good next stop after [`src/writer.md`](writer.md) and
[`src/lib.md`](lib.md), because it introduces **lifetimes** on a struct,
plus your first real trait *implementation*.

```rust
use crate::{attributes::Attributes, element::Element, writer::HtmlWriter};

pub struct Div<'a> {
    element: Element<'a>,
}

impl<'a> Div<'a> {
    pub(crate) fn new(writer: &'a mut HtmlWriter) -> Self {
        Self {
            element: Element::new(writer, "div"),
        }
    }
}

impl<'a> Attributes<'a> for Div<'a> {
    fn element(&mut self) -> &mut Element<'a> {
        &mut self.element
    }
}

impl<'a> Div<'a> {
    pub fn text(&mut self, value: impl std::fmt::Display) {
        self.element.start_children();
        self.element.write_text(value);
    }

    pub fn div<F>(&mut self, children: F)
    where
        F: FnOnce(&mut Div<'_>),
    {
        self.element.start_children();
        let mut div = Div::new(self.element.writer_mut());
        children(&mut div);
        div.finish();
    }

    pub fn finish(self) {
        self.element.finish()
    }
}
```

## Grouped imports and absolute paths

```rust
use crate::{attributes::Attributes, element::Element, writer::HtmlWriter};
```

The `{ }` syntax is just a shorthand for three separate `use` lines —
`use crate::attributes::Attributes;`, `use crate::element::Element;`, and
`use crate::writer::HtmlWriter;` — written together since they share the
`crate::` prefix.

`crate::` is an **absolute path**, always starting from the root of the
current crate (i.e. `src/lib.rs`), regardless of which file you write it
in. It's the same starting point whether you're two files deep or ten. This
contrasts with the *relative* paths you sometimes see elsewhere in Rust
(like `self::` or `super::`), neither of which this crate happens to use.

## A struct with a lifetime parameter

```rust
pub struct Div<'a> {
    element: Element<'a>,
}
```

`<'a>` here is a **lifetime parameter** — not a type, but a name for "how
long some borrowed data is valid for." Lifetime names always start with an
apostrophe; `'a` is just a conventional first choice, the same way you
might call a generic type `T`.

`Div` itself doesn't hold a reference directly. But it wraps an
`Element<'a>` (see [`src/element.md`](element.md)), and `Element` *does*
hold a `&mut HtmlWriter` reference internally. Because `Div` contains
something that borrows data, `Div` has to say so too — the borrow-checker
needs to track, for any given `Div`, how long the data it's ultimately
built on top of stays valid. This is called **threading the lifetime
through**: `Div<'a>` wraps `Element<'a>`, using the *same* `'a`, so the
compiler knows they're tied to the same underlying borrow.

You'll see this pattern repeated on the `impl` block:

```rust
impl<'a> Div<'a> {
```

The `<'a>` after `impl` *declares* the lifetime parameter for this block;
the `<'a>` after `Div` *uses* it, saying "this impl is for `Div`
parameterized by that same `'a`." Both are required — the first
introduces the name, the second refers back to it.

## Constructing a nested builder

```rust
pub(crate) fn new(writer: &'a mut HtmlWriter) -> Self {
    Self {
        element: Element::new(writer, "div"),
    }
}
```

`new` takes a mutable reference to the `HtmlWriter` — with lifetime `'a`,
matching the struct's own parameter — and immediately hands it to
`Element::new`, along with the literal tag name `"div"`. This is
`pub(crate)`, not `pub` (see [`src/writer.md`](writer.md) for what that
visibility level means): the only way to *obtain* a `Div` from outside this
crate is by going through `Html::div` or `Div::div` (see below), never by
calling `Div::new` directly.

## Implementing a trait: `impl Attributes<'a> for Div<'a>`

```rust
impl<'a> Attributes<'a> for Div<'a> {
    fn element(&mut self) -> &mut Element<'a> {
        &mut self.element
    }
}
```

This is different from the plain `impl<'a> Div<'a> { ... }` blocks: the
form `impl<Generics> TraitName for TypeName` implements a **trait** — here,
the `Attributes` trait defined in [`src/attributes.md`](attributes.md) —
*for* `Div`.

`Attributes` requires exactly one method, `element`, and this is where
`Div` provides it: it simply returns a mutable reference to its own
`element` field. That one small method is the *only* thing `Div` has to
supply — but by supplying it, `Div` automatically gains every other method
`Attributes` defines (`id`, `class`, `title`, `attribute`) as free,
ready-to-use methods, with no further code here. See
[`src/attributes.md`](attributes.md) for how that works on the trait's
side.

## Multiple `impl` blocks for the same type

Notice there are *three* `impl` blocks touching `Div` in this file: one for
`Div::new`, one implementing `Attributes for Div`, and one more below for
`text`, `div`, and `finish`. Rust allows any number of `impl` blocks for the
same type (within the same crate) — they all contribute to the same type's
capabilities. Splitting them up like this is purely organizational: it
keeps "how to build one" (`new`), "the shared attribute behavior"
(`Attributes`), and "Div's own behavior" (`text`/`div`/`finish`) visually
separate, even though from the outside it's all just methods on `Div`.

## `impl Trait` in argument position

```rust
pub fn text(&mut self, value: impl std::fmt::Display) {
```

`value: impl std::fmt::Display` means "`value` can be any type that
implements the `Display` trait" (the trait behind `{}` formatting in
`println!` and friends — strings, numbers, and anything with a custom
`Display` impl all qualify). This is called **"impl Trait" in argument
position**, and it's shorthand for a generic parameter you never need to
name elsewhere:

```rust
pub fn text<T: std::fmt::Display>(&mut self, value: T) {
```

Both forms mean the same thing; `impl Trait` is just more concise when, as
here, the type parameter doesn't need to appear anywhere else in the
signature.

The body starts children on the underlying element and writes the escaped
text — see [`src/element.md`](element.md) for `start_children` and
`write_text`, including exactly how and why the text gets HTML-escaped.

## Nesting: `Div`'s own `div` method

```rust
pub fn div<F>(&mut self, children: F)
where
    F: FnOnce(&mut Div<'_>),
{
    self.element.start_children();
    let mut div = Div::new(self.element.writer_mut());
    children(&mut div);
    div.finish();
}
```

This is nearly identical to `Html::div` in [`src/lib.md`](lib.md) — same
generic parameter `F`, same `FnOnce(&mut Div<'_>)` bound, same
closure-calling pattern — but called on a `Div` instead of on `Html`. The
one extra line, `self.element.start_children()`, marks this element as
having a child (which, among other things, writes this `Div`'s own opening
tag if it hasn't been written yet — see
[`src/element.md`](element.md)) before creating the nested builder.

Because `Html` and `Div` both expose a method shaped like this, you can
nest `div` calls as deeply as you like: each one creates a fresh `Div`
scoped to the closure you pass it, and cleans up after itself when the
closure returns — see `examples/nested.md` for this in action across
several levels.

## Consuming `self` again

```rust
pub fn finish(self) {
    self.element.finish()
}
```

Same ownership pattern as `Html::finish` in [`src/lib.md`](lib.md): taking
`self` by value means calling `.finish()` consumes the `Div`, so it can't
accidentally be used again afterwards. Note there's no `-> ReturnType`
here at all — that means the return type is `()`, the unit type, Rust's
way of saying "no meaningful value" (roughly analogous to `void` in other
languages). `self.element.finish()` (no trailing semicolon) is itself an
expression of type `()`, so it's returned as-is.

## See also

- [`src/element.md`](element.md) — what `Element<'a>` actually does
  underneath every `Div`.
- [`src/attributes.md`](attributes.md) — the trait `Div` implements here,
  and why it's structured as default methods over one required method.
- [`src/lib.md`](lib.md) — `Html::div`, the near-twin of this file's `div`
  method, and the FnOnce/closures explanation in full.
