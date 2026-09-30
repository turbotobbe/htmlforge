# `examples/basic.rs` — putting it together

Everything so far ([`src/writer.md`](../src/writer.md),
[`src/lib.md`](../src/lib.md), [`src/div.md`](../src/div.md),
[`src/attributes.md`](../src/attributes.md),
[`src/element.md`](../src/element.md)) was library *implementation*. This
file is where it's actually *used* — a complete, runnable Rust program.

```rust
use htmlforge::{Attributes, Html};

fn main() {
    let mut html = Html::pretty();
    html.div(|div| {
        div.id("foo");
        div.class("bar");

        div.text("Hello");
    });

    let output = html.finish();
    println!("{output}");
}
```

Run it with:

```sh
cargo run --example basic
```

which prints:

```html
<div id="foo" class="bar">
  Hello
</div>
```

## What `examples/` means to Cargo

Any `.rs` file placed directly in an `examples/` directory is automatically
compiled by Cargo as its own small, independent binary, separate from the
library itself. `cargo run --example basic` builds and runs
`examples/basic.rs` specifically — the name after `--example` matches the
filename without its `.rs` extension. This is a convention baked into
Cargo, not something this crate had to configure.

## Importing from an external crate

```rust
use htmlforge::{Attributes, Html};
```

Every file so far used `use crate::...` to reach into *this same* crate's
own modules (see [`src/div.md`](../src/div.md)). This file is different:
it's a separate binary, and it depends on `htmlforge` the way any other
project would — as an external crate. So the path starts with `htmlforge`,
the crate's own name, instead of `crate`. (Cargo treats every `.rs` file
under `examples/` as automatically depending on the library part of the
same package, which is why this works without anything extra in
`Cargo.toml`.)

`{Attributes, Html}` is the same grouped-import shorthand seen in
[`src/div.md`](../src/div.md) — two separate imports written together.
Both are needed: `Html` to actually build a document, and `Attributes`
because `.id(...)` and `.class(...)` are trait methods that only become
callable once the trait itself is in scope — see the "practical gotcha"
section of [`src/attributes.md`](../src/attributes.md) for exactly why.
(`.text(...)`, by contrast, is defined directly on `Div`, not via a trait —
so it would still work even without the `Attributes` import.)

## `fn main`

```rust
fn main() {
    ...
}
```

Every Rust *binary* needs exactly one `fn main()` — it's the entry point,
called automatically when the program starts. (Compare this to
`src/lib.rs`, which has no `main` at all — a library crate doesn't run on
its own, it's meant to be used by other code, like this example.)

## Type inference

```rust
let mut html = Html::pretty();
```

There's no type annotation here — no `let mut html: Html = ...`. Rust
doesn't need one: it looks at the return type of `Html::pretty()` (defined
in [`src/lib.md`](../src/lib.md) as `-> Self`, i.e. `Html`) and infers
`html: Html` automatically. Rust infers types in many situations like this
one; you only need to write them out explicitly when the compiler genuinely
can't work it out on its own, or when you want to be explicit for
readability.

## Passing a closure directly as an argument

```rust
html.div(|div| {
    div.id("foo");
    div.class("bar");

    div.text("Hello");
});
```

`Html::div` (see [`src/lib.md`](../src/lib.md)) expects an argument
matching `FnOnce(&mut Div<'_>)`. Here, `|div| { ... }` is a **closure
literal** written directly inline, with no separate `let` binding — you
don't have to give a closure a name before using it. Its parameter, `div`,
has no type annotation either; the compiler infers `div: &mut Div<'_>`
from `Html::div`'s own bound, the same way it inferred `html`'s type above.

Inside the closure, `div.id(...)` and `div.class(...)` are trait methods
from `Attributes` (see [`src/attributes.md`](../src/attributes.md)), while
`div.text(...)` is an inherent method defined directly on `Div` (see
[`src/div.md`](../src/div.md)) — from the caller's side, they're called
identically, with no visible difference in syntax. The blank line between
`div.class("bar");` and `div.text("Hello");` is purely a formatting choice
the author made; Rust doesn't attach any meaning to blank lines.

Both `"foo"` and `"bar"` are string literals — values of type `&'static
str` (see [`src/element.md`](../src/element.md) for what `'static` means)
— accepted here because `&str` implements `Display`, satisfying the `impl
Display` bound on `id` and `class`.

## Ownership: you can't use `html` after `finish()`

```rust
let output = html.finish();
println!("{output}");
```

`Html::finish` takes `self` by value (see [`src/lib.md`](../src/lib.md)),
so this line *moves* the data out of `html` and into `output`. After this
point, `html` no longer refers to a usable value — if you added a line
like `html.div(...)` after `html.finish()`, the compiler would reject it
with a "use of moved value" error. This is the payoff of the
ownership-based builder pattern used throughout this crate: "finished"
builders can't be accidentally reused, and the compiler enforces it for
you rather than relying on a runtime check.

## String interpolation: `{output}`

```rust
println!("{output}");
```

`println!` is a macro (note the `!`) that formats and prints text.
`{output}` is the **captured identifier** form of interpolation (see
[`src/element.md`](../src/element.md), which uses the same feature in a
panic message): it embeds the `output` variable's value directly into the
string, exactly equivalent to writing `println!("{}", output)` with
`output` passed as a separate argument. Both forms exist in the language;
which one to reach for is mostly a matter of style — `examples/nested.md`
uses the other form, `{}`, for comparison.

## See also

- [`src/lib.md`](../src/lib.md) — `Html::pretty`, `Html::div`, and
  `Html::finish`, all called here.
- `examples/nested.md` — a longer program using the same pieces, plus
  actual nesting.
