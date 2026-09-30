# `src/writer.rs` — the output buffer

This is the simplest file in the crate, which makes it a good place to start
if you're new to Rust. It defines `HtmlWriter`: a plain struct that holds a
growing `String` and a couple of counters, plus a handful of small methods
that mutate it.

```rust
pub struct HtmlWriter {
    output: String,
    pretty: bool,
    depth: usize,
}

impl HtmlWriter {
    pub(crate) fn new(pretty: bool) -> Self {
        Self {
            output: String::new(),
            pretty,
            depth: 0,
        }
    }

    pub(crate) fn finish(self) -> String {
        self.output
    }

    pub(crate) fn write(&mut self, value: &str) {
        self.output.push_str(value);
    }

    pub(crate) fn begin_child(&mut self) {
        if self.pretty && !self.output.is_empty() {
            self.output.push('\n');
            for _ in 0..self.depth {
                self.output.push_str("  ");
            }
        }
    }

    pub(crate) fn indent(&mut self) {
        self.depth += 1;
    }

    pub(crate) fn dedent(&mut self) {
        self.depth -= 1;
    }
}

impl Default for HtmlWriter {
    fn default() -> Self {
        Self::new(false)
    }
}
```

## Struct fields and their types

```rust
pub struct HtmlWriter {
    output: String,
    pretty: bool,
    depth: usize,
}
```

A `struct` groups named pieces of data together. `HtmlWriter` has three
fields:

- `output: String` — an owned, growable, heap-allocated string. (Rust has
  two main string types: `String`, which owns its data and can grow, and
  `&str`, a borrowed *view* into string data you don't own. You'll see both
  throughout this crate.)
- `pretty: bool` — Rust's boolean type, `true` or `false`.
- `depth: usize` — an unsigned integer sized to match the platform's pointer
  width. It's the conventional type for things that count or index
  collections (you'll never have a negative depth, so an unsigned type rules
  out that whole class of bug at compile time).

The struct itself is `pub` (public — visible to anyone depending on this
crate), but none of its *fields* are. That's an important, easy-to-miss
detail: a public struct can still have private fields. Code outside this
module can hold an `HtmlWriter` and call its public methods, but it can't
reach in and read or overwrite `.output` directly. That's how the crate
guarantees the buffer is only ever changed through the controlled methods
below.

## `impl` blocks: attaching functions to a type

```rust
impl HtmlWriter {
    ...
}
```

Functions don't live loose inside a struct definition the way they might in
some languages. Instead, you write a separate `impl` ("implementation")
block and define functions inside it. Every function in `impl HtmlWriter`
is associated with the `HtmlWriter` type.

## `Self` and constructing a struct

```rust
pub(crate) fn new(pretty: bool) -> Self {
    Self {
        output: String::new(),
        pretty,
        depth: 0,
    }
}
```

`Self` (capital S) is shorthand for "whatever type this `impl` block is
for" — here, that's `HtmlWriter`. It saves you from repeating the type
name, and makes the code easier to rename later.

`Self { field: value, ... }` is a *struct literal*: it constructs a new
value of the type. Notice the middle field is written just `pretty,`
instead of `pretty: pretty,` — this is **field init shorthand**. When a
local variable (here, the `pretty` parameter) has the exact same name as
the field you're setting, you can drop the repetition.

`new` has no `self` parameter at all, which makes it an **associated
function** rather than a **method**. You call it as `HtmlWriter::new(false)`
— through the type, not through an existing value — which is exactly the
role a constructor plays. This is a convention, not a language keyword:
Rust doesn't have a built-in notion of "constructor," it's just a function
named `new` by community convention.

## Visibility: `pub(crate)`

Every method here is `pub(crate)` rather than plain `pub`. Rust has a few
visibility levels; the two you'll see in this crate are:

- (no modifier) — private. Visible only in the module that defines it (and
  that module's descendants).
- `pub(crate)` — visible anywhere *inside this crate*, but not to other
  crates that depend on it.
- `pub` — visible to anyone, including external crates.

`HtmlWriter` and its methods are internal plumbing used by `Element`
elsewhere in this crate (see [`src/element.md`](element.md)), but they're
not meant to be part of htmlforge's public API — so `pub(crate)` is the
right level: visible where it's needed, hidden from users of the crate.

## Taking `self` by value vs. by reference

Compare these two methods:

```rust
pub(crate) fn finish(self) -> String {
    self.output
}

pub(crate) fn write(&mut self, value: &str) {
    self.output.push_str(value);
}
```

`finish(self)` takes ownership of the `HtmlWriter` itself — it *consumes*
it. Once you call `.finish()` on a value, that value is gone; you can't use
it again. That's intentional here: finishing means "I'm done building, give
me the result," so there's nothing meaningful left to do with the writer
afterwards. The body `self.output` *moves* the owned `String` out of `self`
and returns it.

`write(&mut self, ...)` instead takes a *mutable borrow* of the writer — a
temporary, exclusive, "I promise to give it back" reference. This lets the
method mutate the writer's fields (via `self.output.push_str(...)`) without
taking ownership, so the caller can go on using the writer afterwards. Most
methods in this crate use `&mut self` for exactly this reason; `self`
(no `&`) is reserved for the handful of "this finishes the builder" methods.

## `String` methods: `push` vs `push_str`

```rust
self.output.push_str(value);   // appends a &str
self.output.push('\n');        // appends a single char
```

`push_str` appends a whole string slice; `push` appends one `char` (a
single Unicode scalar value, written in single quotes). They're different
methods because they take different argument types — Rust doesn't
overload functions by argument type the way some languages do, so the
standard library gives you two clearly-named methods instead.

## `if` with no `else`

```rust
if self.pretty && !self.output.is_empty() {
    ...
}
```

`if` in Rust is an *expression* (it can produce a value, as you'll see with
`match` in [`src/element.md`](element.md)), but it can also be used purely
for its side effects, as a statement, with no `else` branch — which is what
happens here. If the condition is false, the block is simply skipped.

`&&` is logical AND (both sides must be true); `!` is logical NOT. So this
line reads: "only break onto a new line if pretty-printing is enabled *and*
we've already written something" — the second check exists so the very
first thing written to the buffer doesn't get a leading blank line.

## `for` loops over a range, and the `_` placeholder

```rust
for _ in 0..self.depth {
    self.output.push_str("  ");
}
```

`0..self.depth` is a *range* — a value representing "every integer from 0
up to, but not including, `self.depth`." Ranges implement Rust's `Iterator`
trait, so `for` can loop over one directly.

Each time around the loop, `for` would normally bind the current value to a
variable — but this code doesn't care *which* number it's on, only *how
many times* to loop (once per unit of indentation depth). `_` is Rust's
"bind this, but I don't need the name" pattern; it tells both the reader
and the compiler that the loop variable is intentionally unused.

## Compound assignment: `+=` and `-=`

```rust
pub(crate) fn indent(&mut self) {
    self.depth += 1;
}

pub(crate) fn dedent(&mut self) {
    self.depth -= 1;
}
```

`+=` and `-=` are shorthand for `self.depth = self.depth + 1` and
`self.depth = self.depth - 1`. Nothing Rust-specific here — just good to
know the shorthand exists.

## Implementing a standard trait: `Default`

```rust
impl Default for HtmlWriter {
    fn default() -> Self {
        Self::new(false)
    }
}
```

This is different from the `impl HtmlWriter` block above: `impl Default for
HtmlWriter` implements a **trait** — `std::fmt::Default` from the standard
library — *for* this type. A trait is a contract: "any type implementing
`Default` promises to provide a `default()` function that returns an
instance of itself." Implementing standard traits like this is how your own
types plug into the wider Rust ecosystem — any generic code written against
`T: Default` now happily accepts `HtmlWriter` too, and callers can write
`HtmlWriter::default()` as an alternative to `HtmlWriter::new(false)`.

Side note for the curious: every field here (`String`, `bool`, `usize`)
already has its own sensible default (empty string, `false`, `0`) — so
`#[derive(Default)]` above the struct definition would have produced an
identical result automatically. This crate chose to implement it by hand
instead, which you'll see is the same pattern used for `Html` in
[`src/lib.md`](lib.md). You'll meet `#[derive(...)]` properly in
[`src/element.md`](element.md), which uses it for several traits at once.

## See also

- [`src/lib.md`](lib.md) — where `HtmlWriter` gets constructed and owned.
- [`src/element.md`](element.md) — the other side of `begin_child`/`indent`/
  `dedent`: what calls them, and more on `#[derive(...)]`.
