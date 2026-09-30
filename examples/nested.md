# `examples/nested.rs` — nesting, and a couple of loose ends

This builds on `examples/basic.md` — read that one first if you haven't;
this file assumes the basics (importing `htmlforge`, `fn main`, type
inference, closures-as-arguments) are already familiar, and focuses on
what's actually *new* here: nesting elements inside each other, and a
couple of smaller details worth calling out.

```rust
use htmlforge::{Attributes, Html};

fn main() {
    let mut html = Html::pretty();
    html.div(|outer| {
        outer.class("outer");

        outer.div(|inner| {
            inner.class("inner");
            inner.text("Nested content");
        });

        outer.text("After the nested div");

        outer.div(|inner| {
            inner.class("boo");
            inner.class("far");
            inner.text("ole dole doff");
            inner.text("kinke lane koff");
        });

        outer.div(|inner| {
            inner.id("q");
            inner.title("blabla");
        })

    });

    println!("{}", html.finish());
}
```

Run it with:

```sh
cargo run --example nested
```

which prints:

```html
<div class="outer">
  <div class="inner">
    Nested content
  </div>
  After the nested div
  <div class="boo far">
    ole dole doff
    kinke lane koff
  </div>
  <div id="q" title="blabla"></div>
</div>
```

## Nesting: calling `.div(...)` from inside `.div(...)`

```rust
html.div(|outer| {
    ...
    outer.div(|inner| {
        inner.class("inner");
        inner.text("Nested content");
    });
    ...
});
```

The key idea: `Div` exposes its own `.div(...)` method with exactly the
same shape as `Html::div` (see
[`src/div.md`](../src/div.md#nesting-divs-own-div-method) and
[`src/lib.md`](../src/lib.md#generics-and-closures-divf)). So "nesting" in
htmlforge isn't a separate concept you learn once and apply specially —
it's just calling `.div(...)` again, on whichever builder you currently
have in hand, from wherever you happen to be. `outer` here is a `&mut
Div<'_>` (the parameter of the outer closure), and calling `outer.div(...)`
on it works exactly like calling `html.div(...)` did in
`examples/basic.rs` — it opens a new, independent `Div` builder, scoped to
the inner closure, and finishes it automatically when that closure
returns. You could nest a third, fourth, or tenth level exactly the same
way.

## Accumulating classes

```rust
outer.div(|inner| {
    inner.class("boo");
    inner.class("far");
    ...
});
```

Two separate `.class(...)` calls on the same `inner` builder produce
`class="boo far"` in the output, not two separate `class` attributes and
not the second call overwriting the first. This is the accumulating
behavior of `Attributes::class`, built on `Element::append_attribute` —
see [`src/attributes.md`](../src/attributes.md) and
[`src/element.md`](../src/element.md#pattern-matching-with-match-on-an-option)
for exactly how that's implemented. Contrast this with `.id(...)`, called
once per element two blocks later (`inner.id("q")`) — calling `.id(...)`
*twice* on the same element, unlike `.class(...)`, would panic (see
[`src/attributes.md`](../src/attributes.md)).

## Calling `.text(...)` more than once

```rust
outer.div(|inner| {
    ...
    inner.text("ole dole doff");
    inner.text("kinke lane koff");
});
```

Unlike `id`, there's nothing that restricts `.text(...)` to being called
once. Each call writes another text node as a child of `inner`, in order —
which is why the output shows both strings on their own lines, one after
the other, rather than one call overwriting the other.

## Plain sequential statements

```rust
outer.div(|inner| { ... });

outer.text("After the nested div");

outer.div(|inner| { ... });

outer.div(|inner| { ... })
```

There's nothing special connecting these four calls on `outer` — they're
just ordinary, separate statements, one after another, each fully
completed before the next begins. Rust doesn't require you to chain method
calls together; writing them as a sequence of statements like this works
just as well, and the order they appear in source is the order the
corresponding elements and text appear in the output.

## The missing semicolon at the end

```rust
outer.div(|inner| {
    inner.id("q");
    inner.title("blabla");
})

});
```

Look closely: the last `outer.div(...)` call has **no semicolon** after
its closing `)`, unlike every other statement in this function. This is
legal Rust, and it's worth understanding why, since it looks like it might
be a typo.

A block's overall value is its final expression, *if* that expression
isn't terminated by a semicolon — this is the same rule that made
`escaped` (with no semicolon) the return value of `escape_html` in
[`src/element.md`](../src/element.md#a-free-function-escape_html). Here,
the outer closure's body is a block, and its last expression is
`outer.div(|inner| { ... })`. Since `Div::div` returns `()` (the unit
type — see [`src/div.md`](../src/div.md#consuming-self-again)), that
expression's value is `()`. The closure itself needs to return `()` anyway,
to satisfy `FnOnce(&mut Div<'_>)`'s implied return type. So whether or not
you write the trailing semicolon here makes no *functional* difference —
with it, the statement's value `()` is discarded and the block falls
through to an implicit `()`; without it, the block's value already *is*
that same `()`. Most Rust code (and tools like `rustfmt`) would add the
semicolon anyway for consistency, but the compiler doesn't require it in a
case like this, and this file happens to leave it out.

## `{}` vs. `{output}`

```rust
println!("{}", html.finish());
```

`examples/basic.rs` used the captured-identifier form,
`println!("{output}")` (see
`examples/basic.md#string-interpolation-output`). This file uses the other,
more traditional form instead: `{}` is a placeholder, and the value to fill
it with — `html.finish()` — is passed as a separate, second argument to the
macro. Both forms are valid Rust; `{}` is required when the thing you want
to print isn't a simple variable name (like the direct method-call result
here), since you can't write `html.finish()` inside `{}` directly.

## See also

- `examples/basic.md` — the simpler example this one builds on.
- [`src/div.md`](../src/div.md) — `Div::div`, `Div::text`, and why
  `Div::finish` returns `()`.
- [`src/attributes.md`](../src/attributes.md) — accumulating `.class(...)`
  vs. once-only `.id(...)`.
