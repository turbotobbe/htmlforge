# `src/lib.rs` — the crate root

`lib.rs` is a special filename: Cargo treats it as the root of a *library*
crate (as opposed to `main.rs`, which would make this a *binary*). Every
other module in `src/` is wired together from here, and whatever is `pub`
at this top level is what users of the crate — including the examples in
`examples/` — can actually see and use.

```rust
mod attributes;
mod div;
mod element;
mod writer;

pub use attributes::Attributes;
pub use div::Div;
pub use element::Element;

use writer::HtmlWriter;

pub struct Html {
    writer: HtmlWriter,
}

impl Html {
    pub fn new() -> Self {
        Self {
            writer: HtmlWriter::new(false),
        }
    }

    pub fn pretty() -> Self {
        Self {
            writer: HtmlWriter::new(true),
        }
    }

    pub fn finish(self) -> String {
        self.writer.finish()
    }

    pub(crate) fn writer_mut(&mut self) -> &mut HtmlWriter {
        &mut self.writer
    }

    pub fn div<F>(&mut self, children: F)
    where
        F: FnOnce(&mut Div<'_>),
    {
        let mut div = Div::new(self.writer_mut());
        children(&mut div);
        div.finish();
    }
}

impl Default for Html {
    fn default() -> Self {
        Self::new()
    }
}
```

## The module tree: `mod`

```rust
mod attributes;
mod div;
mod element;
mod writer;
```

Each `mod name;` line (no `{ ... }` body) tells the compiler "there's a
module called `name`, and its contents live in `src/name.rs`." This is how
a Rust *crate* — the whole `htmlforge` library — is built up from several
files: `lib.rs` declares the module tree, and each module's own file (like
`src/div.rs`) fills in that module's contents.

Like struct fields, modules are private by default. Declaring `mod div;`
here (with no `pub`) means the `div` module — and everything in it that
isn't separately re-exported — is only reachable from inside this crate,
not from crates that depend on `htmlforge`.

## Re-exporting with `pub use`

```rust
pub use attributes::Attributes;
pub use div::Div;
pub use element::Element;

use writer::HtmlWriter;
```

`use` brings an item into scope under a shorter path — instead of writing
`div::Div` everywhere in this file, `use div::Div;` (implied by `pub use`
here) lets you just write `Div`.

The `pub` in front of `use` does something extra: it **re-exports** the
item. Without `pub`, a `use` is private — it's a convenience for *this
file only*. With `pub`, the item becomes part of `htmlforge`'s own public
API at the crate root, which is exactly what lets the examples write:

```rust
use htmlforge::{Attributes, Html};
```

instead of the more awkward `use htmlforge::div::Div;`. This "flatten the
public API at the crate root" pattern is extremely common in Rust
libraries.

Notice `HtmlWriter` is imported with a plain `use`, not `pub use` — it's
needed inside this file (the `Html` struct holds one), but it's
intentionally *not* re-exported. Combined with `mod writer;` having no
`pub`, this means `HtmlWriter` is invisible outside the crate entirely —
it's pure internal implementation detail. Compare this to `Div` and
`Element`, which *are* re-exported because users need to name their types
(for example, in the `FnOnce(&mut Div<'_>)` bound below).

## A public struct with a private field

```rust
pub struct Html {
    writer: HtmlWriter,
}
```

Same pattern as `HtmlWriter` itself in [`src/writer.md`](writer.md): `Html`
is public, but its one field isn't. Nothing outside this crate can
construct an `Html` directly with a struct literal (`Html { writer: ... }`)
— they're forced to go through `Html::new()` or `Html::pretty()` below,
which is exactly the point: it lets the crate guarantee every `Html` starts
from a valid, consistent state.

## Two constructors, one real difference

```rust
impl Html {
    pub fn new() -> Self {
        Self {
            writer: HtmlWriter::new(false),
        }
    }

    /// Like [`Html::new`], but indents nested elements and breaks each
    /// element and text node onto its own line.
    pub fn pretty() -> Self {
        Self {
            writer: HtmlWriter::new(true),
        }
    }
    ...
}
```

Both are associated functions (no `self` parameter — see
[`src/writer.md`](writer.md) for that distinction), called as
`Html::new()` / `Html::pretty()`. They only differ in the `bool` they pass
to `HtmlWriter::new`, which controls pretty-printing — see
[`src/writer.md`](writer.md) for what that flag actually changes.

The `///` comment above `pretty` is a **doc comment**, not an ordinary `//`
comment. Doc comments are extracted by `rustdoc` to build the documentation
you see on [docs.rs](https://docs.rs) — an ordinary `//` comment wouldn't
show up there at all. The `[`Html::new`]` inside it is *intra-doc link*
syntax: rustdoc turns it into a clickable link to the `Html::new` item's
own docs page.

## Consuming `self` to "finish" the builder

```rust
pub fn finish(self) -> String {
    self.writer.finish()
}
```

`finish` takes `self` by value, so calling it consumes the `Html` —
exactly as `HtmlWriter::finish` consumes the writer (see
[`src/writer.md`](writer.md)). `self.writer` is moved out of `self` and its
own `.finish()` is called on it, producing the final `String`. After you
call `html.finish()`, the `html` variable is gone — you'd get a compiler
error if you tried to use it again. This is a deliberate design choice,
often called "the builder pattern": the type system itself prevents you
from accidentally reusing a finished builder.

## Returning a mutable reference, and lifetime elision

```rust
pub(crate) fn writer_mut(&mut self) -> &mut HtmlWriter {
    &mut self.writer
}
```

This borrows `self` mutably and hands back a mutable reference to the
`writer` field inside it. You might expect a lifetime annotation on that
return type, since it's a reference — but Rust applies a set of **lifetime
elision rules** for common patterns like this one: a method with exactly
one reference input (`&mut self`) and a reference output is assumed to
return something borrowed *from* that input, so the compiler fills in the
lifetime for you. Written out in full, the signature would be
`fn writer_mut<'x>(&'x mut self) -> &'x mut HtmlWriter`.

## Generics and closures: `div<F>`

```rust
pub fn div<F>(&mut self, children: F)
where
    F: FnOnce(&mut Div<'_>),
{
    let mut div = Div::new(self.writer_mut());
    children(&mut div);
    div.finish();
}
```

This is the method that actually builds an element, and it introduces two
big Rust ideas at once.

**Generics.** `<F>` declares a *generic type parameter* named `F` — a
placeholder standing in for "some type, to be determined by the caller."
The `where F: FnOnce(&mut Div<'_>)` clause is a **trait bound**: it
restricts `F` to only those types that implement the `FnOnce` trait with
that specific signature. (You could also write the bound inline as
`fn div<F: FnOnce(&mut Div<'_>)>(...)`; the `where` form is just a more
readable style, especially once bounds get longer.)

**Closures.** The types that satisfy `FnOnce(&mut Div<'_>)` are *closures*
— anonymous, inline functions — that take one argument (`&mut Div<'_>`) and
return nothing. Rust actually has three closure traits, forming a
hierarchy of "how the closure is allowed to use what it captures":

- `Fn` — can be called repeatedly, and only borrows its captured
  environment immutably.
- `FnMut` — can be called repeatedly, and may mutate what it captures.
- `FnOnce` — can only be called *once*, because calling it may consume
  (move out of) what it captures.

`FnOnce` is the loosest bound — every closure implements at least `FnOnce`
— and it's exactly what's needed here: `div` only ever calls `children`
one time (see the body below), so there's no reason to demand anything
stricter.

`Div<'_>` uses the **elided lifetime**, written `'_`. It means "there is a
lifetime parameter here, but let the compiler work out what it is from
context" — you'll see `Div`'s actual lifetime parameter, and what it's
for, in [`src/div.md`](div.md).

**The body.** `let mut div = Div::new(self.writer_mut());` creates a new
`Div` builder that borrows the writer (via the `writer_mut` method just
above). `mut` is required here because the next line calls methods that
need `&mut div`. `children(&mut div);` then *calls* the closure like a
function, handing it a mutable reference to the new builder — this is the
callback pattern: instead of `div` returning something the caller has to
remember to finish themselves, the caller's closure is invoked in the
middle of `div`'s own logic, and `div.finish();` runs automatically
afterwards regardless of what the closure did inside. See
[`src/div.md`](div.md) for the `Div` side of this same pattern, since `Div`
has its own, near-identical `div` method for nesting.

## `impl Default for Html`

```rust
impl Default for Html {
    fn default() -> Self {
        Self::new()
    }
}
```

Same idea as `impl Default for HtmlWriter` in
[`src/writer.md`](writer.md): this lets `Html::default()` work as an
alternative spelling of `Html::new()`, and lets any generic code that
requires `T: Default` accept `Html` too.

## See also

- [`src/writer.md`](writer.md) — the simplest file in the crate; start
  there if any of the struct/`impl`/`Self` basics above felt unfamiliar.
- [`src/div.md`](div.md) — the lifetime on `Div<'a>`, and the matching
  `div` method that lets you *nest* elements.
- `examples/basic.md` and `examples/nested.md` — this exact `Html::div`
  method, called from real code.
