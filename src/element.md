# `src/element.rs` — the shared machinery underneath every element

`Element` is where most of the crate's actual work happens: writing tags,
tracking attributes, and escaping text. It's also the file with the widest
range of Rust concepts, so it's worth reading after the others — especially
[`src/div.md`](div.md) and [`src/attributes.md`](attributes.md), since this
file is what their methods ultimately delegate to.

```rust
use std::fmt::Display;

use crate::writer::HtmlWriter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementPhase {
    Attributes,
    Children,
}

pub struct Element<'a> {
    writer: &'a mut HtmlWriter,
    tag: &'static str,
    attributes: Vec<(String, String)>,
    phase: ElementPhase,
}

impl<'a> Element<'a> {
    pub(crate) fn new(writer: &'a mut HtmlWriter, tag: &'static str) -> Self {
        Self {
            writer,
            tag,
            attributes: Vec::new(),
            phase: ElementPhase::Attributes,
        }
    }

    pub(crate) fn writer_mut(&mut self) -> &mut HtmlWriter {
        &mut *self.writer
    }

    fn write_open_tag(&mut self) {
        self.writer.begin_child();
        self.writer.write("<");
        self.writer.write(self.tag);

        for (name, value) in &self.attributes {
            self.writer.write(" ");
            self.writer.write(name);
            self.writer.write("=\"");
            self.writer.write(&escape_html(value));
            self.writer.write("\"");
        }

        self.writer.write(">");
    }

    pub(crate) fn start_children(&mut self) {
        if self.phase == ElementPhase::Attributes {
            self.write_open_tag();
            self.writer.indent();
            self.phase = ElementPhase::Children;
        }
    }

    pub(crate) fn finish(mut self) {
        let had_children = self.phase == ElementPhase::Children;
        self.start_children();
        self.writer.dedent();

        if had_children {
            self.writer.begin_child();
        }

        self.writer.write("</");
        self.writer.write(self.tag);
        self.writer.write(">");
    }

    pub(crate) fn assert_attributes_allowed(&self) {
        assert_eq!(
            self.phase,
            ElementPhase::Attributes,
            "HTML attributes must be declared before children"
        )
    }

    pub(crate) fn set_attribute(&mut self, name: &str, value: impl Display) {
        let value = value.to_string();
        match self.attributes.iter_mut().find(|(n, _)| n == name) {
            Some(existing) => existing.1 = value,
            None => self.attributes.push((name.to_string(), value)),
        }
    }

    pub(crate) fn set_attribute_once(&mut self, name: &str, value: impl Display) {
        assert!(
            !self.attributes.iter().any(|(n, _)| n == name),
            "the \"{name}\" attribute must only be set once"
        );
        self.set_attribute(name, value);
    }

    pub(crate) fn append_attribute(&mut self, name: &str, value: impl Display, separator: &str) {
        let value = value.to_string();
        match self.attributes.iter_mut().find(|(n, _)| n == name) {
            Some(existing) => {
                existing.1.push_str(separator);
                existing.1.push_str(&value);
            }
            None => self.attributes.push((name.to_string(), value)),
        }
    }

    pub(crate) fn write_text(&mut self, value: impl Display) {
        self.writer.begin_child();
        self.writer.write(&escape_html(&value.to_string()));
    }
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}
```

## Importing from the standard library vs. your own crate

```rust
use std::fmt::Display;

use crate::writer::HtmlWriter;
```

`std::fmt::Display` comes from `std`, Rust's standard library — always
available, no dependency needed. `crate::writer::HtmlWriter` is an
absolute path into this crate's own module tree (see
[`src/div.md`](div.md) for more on `crate::` paths). Seeing both side by
side is a good moment to notice they look almost identical — `use` doesn't
care whether a path starts at the standard library or your own code, it's
all just "paths to items."

## `#[derive(...)]`: generating trait implementations automatically

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementPhase {
    Attributes,
    Children,
}
```

The `#[...]` syntax is an **attribute** — metadata attached to the item
below it. `derive` is a particular attribute that auto-generates trait
implementations, instead of you writing them by hand. This one line saves
five separate `impl ... for ElementPhase { ... }` blocks. Each trait here
does something different:

- **`Debug`** enables `{:?}` formatting (as opposed to `{}`, which needs
  `Display`) — mostly used for diagnostics and debugging, e.g. printing a
  value while tracking down a bug.
- **`Clone`** adds an explicit `.clone()` method, letting you make an
  independent duplicate of a value on demand.
- **`Copy`** goes further: it marks the type as safe to duplicate
  *implicitly*, via simple bit-for-bit copying, every time it would
  otherwise be *moved* (e.g. passed by value, or assigned to another
  variable). Only simple, stack-only data can be `Copy` — nothing that
  owns a heap allocation (like `String` or `Vec`) is allowed to implement
  it. `ElementPhase` qualifies because it's just a plain enum with no
  fields. Rust requires any `Copy` type to also implement `Clone` (`Copy`
  is a stricter promise layered on top), which is why both are listed
  together.
- **`PartialEq`** enables the `==` and `!=` operators.
- **`Eq`** is a *marker trait* — it adds no methods of its own, but
  declares that equality is total and reflexive (every value equals
  itself, including in edge cases like floating-point `NaN`, which
  notoriously breaks this and so can only implement `PartialEq`, never
  `Eq`). It's required by some standard library APIs (like using a value
  as a `HashMap` key), though this crate doesn't do that here — it's
  included simply because it's true of this type and costs nothing to
  state.

## `enum`: a type with a fixed set of named variants

```rust
pub enum ElementPhase {
    Attributes,
    Children,
}
```

An `enum` defines a type that can be *one of* several named **variants** —
here, either `ElementPhase::Attributes` or `ElementPhase::Children`, never
anything else and never both at once. This could have been a `bool`
instead, but a two-variant enum like this documents *what* the two states
mean directly in the type, rather than leaving a reader to guess what
`true` and `false` stand for. You'll see this enum used purely as a state
flag throughout the rest of the file.

## The `Element` struct: three different kinds of "how long is this valid"

```rust
pub struct Element<'a> {
    writer: &'a mut HtmlWriter,
    tag: &'static str,
    attributes: Vec<(String, String)>,
    phase: ElementPhase,
}
```

- `writer: &'a mut HtmlWriter` — a mutable reference, *borrowed*, tied to
  the struct's own lifetime parameter `'a` (see [`src/div.md`](div.md) for
  the introduction to lifetime parameters on structs). This is the
  reference threaded all the way from `Html`/`Div` down into here.
- `tag: &'static str` — a reference with the special `'static` lifetime,
  meaning "valid for the entire remaining life of the program." String
  *literals* (text written directly in the source, like `"div"` in
  [`src/div.md`](div.md)'s `Element::new(writer, "div")`) are embedded
  directly in the compiled binary and are always `'static` — there's never
  a moment where a string literal becomes invalid, so the type system
  reflects that.
- `attributes: Vec<(String, String)>` — a `Vec<T>` is a growable,
  heap-allocated array. Here `T` is `(String, String)`, a **tuple**: a
  fixed-size, ordered grouping of values — in this case, an attribute name
  paired with its value, both owned `String`s. A `Vec` of pairs was chosen
  over something like a `HashMap<String, String>` because it preserves
  **insertion order** — attributes come out in the order you set them,
  which matters for predictable, reproducible HTML output — and a linear
  scan is perfectly fast for the small number of attributes a single
  element typically has.
- `phase: ElementPhase` — the enum defined just above, used as a simple
  state flag (explained more under `start_children` below).

## Reborrowing: `&mut *self.writer`

```rust
pub(crate) fn writer_mut(&mut self) -> &mut HtmlWriter {
    &mut *self.writer
}
```

`self.writer` already has type `&'a mut HtmlWriter` — it's *already* a
mutable reference. So why not just `return self.writer`? Because `self`
here is only borrowed (`&mut self`, not owned `self`) — moving `self.writer`
out of it would mean taking ownership of data behind a borrow, which Rust
forbids.

The fix is `&mut *self.writer`: dereference with `*self.writer` to get at
the `HtmlWriter` the reference points to, then immediately take a fresh
`&mut` reference to it. This is called **reborrowing** — producing a new,
shorter-lived mutable reference *from* an existing one, without moving the
original out of `self`. It's a common pattern any time you need to "hand
out" a field that's itself a `&mut` reference.

## A genuinely private method

```rust
fn write_open_tag(&mut self) {
```

No `pub` or `pub(crate)` at all — this one is private to the `element`
module itself (contrast with the other methods here, which are
`pub(crate)` — see [`src/writer.md`](writer.md) for what that visibility
level means). Nothing outside this file can call `write_open_tag`
directly; it's purely an internal helper for `start_children` below.

## Iterating over a `Vec` by reference, and destructuring tuples

```rust
for (name, value) in &self.attributes {
    self.writer.write(" ");
    self.writer.write(name);
    ...
}
```

`&self.attributes` produces a shared reference to the whole `Vec`. A `for`
loop over that reference iterates once per element, handing you a
reference to each one — here, each element is a `(String, String)` tuple,
so you get a `&(String, String)`.

The pattern `(name, value)` **destructures** that tuple: it pulls the two
elements apart and binds them to two separate names. Written naively you
might expect to need `&(name, value)` or manual dereferencing, but Rust's
**match ergonomics** let you write the pattern as if you already had the
tuple by value, and the compiler automatically adjusts the bindings —
`name` and `value` each end up as `&String`, references into the original
data, with nothing copied or moved. This loop only reads `self.attributes`,
so it never needs to mutate it.

## Deref coercion: passing `&String` where `&str` is expected

```rust
self.writer.write(&escape_html(value));
```

`value` here has type `&String` (from the destructuring above), but
`escape_html` is declared as `fn escape_html(value: &str) -> String`. This
compiles because `String` implements `Deref<Target = str>`, and Rust
applies **deref coercion** automatically at call sites like this one: a
`&String` is silently converted to a `&str` wherever one is expected. This
is why you'll rarely see explicit conversions between the two in idiomatic
Rust code — the compiler bridges the gap for you.

## Comparing enum variants with `==`

```rust
pub(crate) fn start_children(&mut self) {
    if self.phase == ElementPhase::Attributes {
        self.write_open_tag();
        self.writer.indent();
        self.phase = ElementPhase::Children;
    }
}
```

`self.phase == ElementPhase::Attributes` only compiles because of the
derived `PartialEq` on `ElementPhase` (see above) — without it, `enum`
values can't be compared with `==` at all.

This method is the heart of the crate's lazy tag-writing design: an
`Element` starts in the `Attributes` phase, and the very first time
anything tries to add a child (a nested element or a text node), this
method writes the opening tag — with whatever attributes have been set so
far — then flips `self.phase` to `Children`. Every subsequent call to
`start_children` on the same element sees `phase` is no longer
`Attributes`, so the `if` body is skipped: the tag only gets written once,
no matter how many children follow it.

## Three flavors of `self`, all in one file

You've now seen every way a method can take `self` in Rust, and this file
uses all three:

- `&self` — a shared, read-only borrow. Used by
  `assert_attributes_allowed(&self)`, which only needs to *read*
  `self.phase`.
- `&mut self` — a mutable borrow. Used by most methods here
  (`start_children`, `set_attribute`, `write_text`, ...), which need to
  change fields on `self` but shouldn't take ownership away from the
  caller.
- `self` / `mut self` — takes ownership outright, consuming the value (see
  `finish` next). `mut self` additionally makes the local binding
  mutable, which matters when, as below, the method body needs to call
  other `&mut self` methods on itself.

## `finish(mut self)`

```rust
pub(crate) fn finish(mut self) {
    let had_children = self.phase == ElementPhase::Children;
    self.start_children();
    self.writer.dedent();

    if had_children {
        self.writer.begin_child();
    }

    self.writer.write("</");
    self.writer.write(self.tag);
    self.writer.write(">");
}
```

Note the parameter is `mut self`, not just `self`. Taking `self` by value
already means this method owns the `Element` outright (matching
`Div::finish` and `Html::finish` — see [`src/div.md`](div.md) and
[`src/lib.md`](lib.md)); the extra `mut` is needed *in addition*, because
the body goes on to call `self.start_children()`, which requires `&mut
self`. Without `mut` on the parameter, `self` would be an immutable local
binding, and taking a mutable reference to it (even from inside its own
method) wouldn't be allowed.

The logic: check *before* anything else whether this element already had
children (`had_children`); call `start_children()` unconditionally, which
guarantees the opening tag gets written even for an element with *no*
children at all (so `<div></div>` is valid output, not just `<div>` with
a missing close); reduce the indentation level; and only insert a
newline-and-indent *before* the closing tag if there actually were
children (an empty element's open and close tags stay on the same line).
Finally, the closing tag itself is written directly with three separate
`self.writer.write(...)` calls.

## `assert_eq!` and `assert!`

```rust
pub(crate) fn assert_attributes_allowed(&self) {
    assert_eq!(
        self.phase,
        ElementPhase::Attributes,
        "HTML attributes must be declared before children"
    )
}
```

```rust
assert!(
    !self.attributes.iter().any(|(n, _)| n == name),
    "the \"{name}\" attribute must only be set once"
);
```

`assert_eq!` and `assert!` are macros (note the `!`) built into the
standard library. `assert_eq!(left, right, message)` panics — immediately
stopping the program with an error — if `left != right`, printing your
custom message alongside the usual "these two values weren't equal"
output. `assert!(condition, message)` is the more general form: it panics
if `condition` is `false`. Both are used here to enforce this crate's
runtime invariants (the "attributes before children" rule, and the
"`id` can only be set once" rule) — violating them is treated as a
programmer error, not something to recover from gracefully, so a panic
(rather than returning a `Result`) is the appropriate tool. This is also
documented behavior — see this crate's own [`README.md`](../README.md).

Note the string literal's escaped quotes, `\"{name}\"` — `\"` inside a
`"..."` string is how you include a literal double-quote character without
ending the string early. `{name}` is a **captured identifier** in a format
string (stabilized in the 2021 edition): it embeds the `name` variable
directly, equivalent to writing `"the \"{}\" attribute..."` as a separate
format string with `name` passed as an extra argument.

## `Iterator::find` and `Iterator::any`

```rust
pub(crate) fn set_attribute(&mut self, name: &str, value: impl Display) {
    let value = value.to_string();
    match self.attributes.iter_mut().find(|(n, _)| n == name) {
        Some(existing) => existing.1 = value,
        None => self.attributes.push((name.to_string(), value)),
    }
}
```

```rust
assert!(
    !self.attributes.iter().any(|(n, _)| n == name),
    ...
);
```

`.iter()` produces an iterator of shared references (`&(String, String)`);
`.iter_mut()` produces an iterator of *mutable* references
(`&mut (String, String)`). `set_attribute` needs `iter_mut` because, if it
finds an existing attribute with the same name, it needs to *modify* it in
place; `set_attribute_once` only needs to check whether one exists, so the
cheaper `iter()` (paired with `.any(...)`) is enough.

Both calls pass a **closure as a predicate**: `|(n, _)| n == name`. This
closure takes one parameter — a reference to a tuple — destructured
directly in the closure's parameter list, the same pattern used in the
`for` loop above. `_` ignores the second element of the tuple (the current
value) since only the name is being compared.

- `.find(predicate)` returns an `Option<Item>`: `Some(item)` for the first
  matching element, or `None` if nothing matched.
- `.any(predicate)` returns a plain `bool`: `true` if *any* element
  matches, `false` otherwise.

## Pattern matching with `match` on an `Option`

```rust
match self.attributes.iter_mut().find(|(n, _)| n == name) {
    Some(existing) => existing.1 = value,
    None => self.attributes.push((name.to_string(), value)),
}
```

`Option<T>` is the standard library's way of representing "a value that
might not be there" — it has exactly two variants, `Some(T)` (a value is
present) and `None` (it isn't). `match` lets you handle each possibility
explicitly: if `find` returned `Some(existing)`, `existing` is bound to the
found `&mut (String, String)`, and `existing.1 = value` overwrites its
second element (`.0` and `.1` access tuple elements by position, the way
you'd index an array — but tuple elements can have different types, so
they're accessed by position rather than a runtime index). If it returned
`None`, a brand-new tuple is pushed onto the `Vec` instead. Unlike the `if`
seen in [`src/writer.md`](writer.md), this `match` doesn't produce a value
here (both arms end in `;`-terminated statements) — but `match` is capable
of being an expression too; you'll see that below in `escape_html`.

## Shadowing

```rust
pub(crate) fn set_attribute(&mut self, name: &str, value: impl Display) {
    let value = value.to_string();
    ...
}
```

The parameter `value` has type `impl Display` — some generic, printable
type. `let value = value.to_string();` declares a *new* variable, also
named `value`, that **shadows** the original: from this line onward,
`value` refers to the new `String`, and the original parameter is no
longer accessible by that name. This is different from mutating a
variable — it's a fresh binding, allowed to even have a different type
than what it's shadowing, which is exactly what happens here (`impl
Display` in, `String` out).

## `ToString`, via `Display`

```rust
let value = value.to_string();
```

`.to_string()` comes from the `ToString` trait — and the standard library
provides a **blanket implementation**: *any* type implementing `Display`
automatically gets `ToString` for free, with `to_string()` built on top of
however that type formats itself. This is why `set_attribute`,
`set_attribute_once`, and `write_text` can all accept a generic `impl
Display` value and turn it into an owned `String` with the same one-line
call, no matter what concrete type was actually passed in.

## A free function: `escape_html`

```rust
fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}
```

This function lives outside any `impl` block entirely — a plain,
module-private function (no `pub`), not a method on any type. Rust doesn't
require every function to belong to a type the way some languages do.

- `String::with_capacity(value.len())` preallocates enough space for a
  string at least as long as the input, up front. This is a minor
  performance optimization: since escaping can only ever make the string
  the same length or longer (never shorter), and most input is `&`-free,
  reserving `value.len()` bytes upfront avoids some of the reallocation
  that a bare `String::new()` would need to do as characters are pushed.
- `.chars()` iterates over the string's **Unicode scalar values** (`char`s)
  one at a time — not raw bytes, which matters for any text containing
  multi-byte characters (accented letters, emoji, non-Latin scripts, and
  so on).
- `match c { '&' => ..., '<' => ..., ..., _ => escaped.push(c) }` pattern
  matches the current character against each special case that needs
  escaping. Character literals in single quotes (`'&'`, `'<'`, ...) are
  matched exactly; `_` is the wildcard arm, catching every character that
  isn't one of the five listed, and pushing it through unchanged.
- The function's final line, `escaped` — with no semicolon and no
  `return` keyword — is an **implicit return**: in Rust, a block's value
  is its last expression, as long as that expression isn't followed by a
  `;`. Adding a semicolon there would turn `escaped` into a statement
  (discarding its value) and make the function fail to compile, since it's
  declared to return a `String`.

This is also the function referenced from
[`src/div.md`](div.md)'s `text` method and
[`src/attributes.md`](attributes.md)'s default methods — every attribute
value and every text node passed to htmlforge flows through here before
it's written, which is what makes the escaping automatic and impossible to
accidentally skip.

## See also

- [`src/writer.md`](writer.md) — `begin_child`, `indent`, and `dedent`,
  called throughout this file.
- [`src/div.md`](div.md) — the one type wrapping `Element` today, and
  where `"div"` (a `&'static str`) comes from.
- [`src/attributes.md`](attributes.md) — the trait whose default methods
  call `assert_attributes_allowed`, `set_attribute`, `set_attribute_once`,
  and `append_attribute`.
