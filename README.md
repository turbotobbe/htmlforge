# htmlforge

[![Crates.io](https://img.shields.io/crates/v/htmlforge.svg)](https://crates.io/crates/htmlforge)
[![Documentation](https://docs.rs/htmlforge/badge.svg)](https://docs.rs/htmlforge)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![GitHub](https://img.shields.io/badge/github-turbotobbe%2Fhtmlforge-8da0cb.svg)](https://github.com/turbotobbe/htmlforge)

A small, type-safe, closure-based builder for constructing HTML strings in
Rust.

Instead of concatenating strings or writing a template file, you describe a
document as nested Rust closures. htmlforge takes care of opening and closing
tags, indentation, and HTML-escaping for you:

```rust
use htmlforge::{Attributes, Html};

let mut html = Html::pretty();
html.div(|div| {
    div.id("foo");
    div.class("bar");
    div.text("Hello");
});

println!("{}", html.finish());
```

```html
<div id="foo" class="bar">
  Hello
</div>
```

> **Status:** htmlforge is at an early alpha stage (`0.1.0`). The API surface
> is intentionally small right now — see [Current limitations](#current-limitations)
> below for what isn't supported yet.

## Contents

- [Installation](#installation)
- [Running the bundled examples](#running-the-bundled-examples)
- [Guide: using htmlforge in your own project](#guide-using-htmlforge-in-your-own-project)
  - [1. Add the dependency and import the trait](#1-add-the-dependency-and-import-the-trait)
  - [2. Start a document](#2-start-a-document-html)
  - [3. Build elements with closures](#3-build-elements-with-closures)
  - [4. Set attributes](#4-set-attributes)
  - [5. Add text](#5-add-text-escaping-is-automatic)
  - [6. Nest elements](#6-nest-elements)
  - [7. Get the finished string, and do something with it](#7-get-the-finished-string-and-do-something-with-it)
- [Current limitations](#current-limitations)
- [Minimum supported Rust version](#minimum-supported-rust-version)
- [License](#license)

## Installation

Add htmlforge to your `Cargo.toml`:

```toml
[dependencies]
htmlforge = "0.1"
```

or let Cargo do it for you:

```sh
cargo add htmlforge
```

## Running the bundled examples

```sh
git clone https://github.com/turbotobbe/htmlforge.git
cd htmlforge
cargo run --example basic
cargo run --example nested
```

- [`examples/basic.rs`](examples/basic.rs) shows the minimal case: one `div`
  with an `id`, a `class`, and a text node.
- [`examples/nested.rs`](examples/nested.rs) shows nested `div`s, multiple
  text nodes, and accumulating multiple `class` calls on the same element.

## Guide: using htmlforge in your own project

This is the part that matters once htmlforge is a dependency rather than a
checkout — how to actually build documents with it.

### 1. Add the dependency and import the trait

```rust
use htmlforge::{Attributes, Html};
```

Two things to note:

- `Html` is the type you construct a document with.
- `Attributes` is a **trait** that adds `.id(...)`, `.class(...)`,
  `.title(...)`, and `.attribute(...)` to element builders. Rust trait
  methods are only visible when the trait is in scope, so if you forget this
  import you'll get a compiler error like `no method named 'id' found for
  mutable reference to 'Div' in the current scope` — importing `Attributes`
  fixes it.

### 2. Start a document (`Html`)

```rust
let mut html = Html::new();     // compact: no indentation, no line breaks
let mut html = Html::pretty();  // indented, one node per line
```

`Html::new()` and `Html::pretty()` (and `Html::default()`, which is the same
as `Html::new()`) are the only two ways to start a document, and the choice
is fixed for the lifetime of that `Html` value — you can't toggle it midway
through.

### 3. Build elements with closures

Elements are added by calling a method that takes a closure. The closure
receives a `&mut` builder for that element, and the element is automatically
closed (its closing tag written) as soon as the closure returns:

```rust
html.div(|div| {
    div.text("Hello");
});
```

You never hold on to the child builder outside of the closure — this is what
guarantees every element you open gets properly closed, without you having
to remember to call a `.close()` or `.end()` method yourself.

### 4. Set attributes

Attribute methods come from the `Attributes` trait, so `Attributes` must be
in scope (see step 1):

```rust
html.div(|div| {
    div.id("main");           // sets the `id` attribute
    div.class("card");        // sets/adds to the `class` attribute
    div.class("card--active"); // repeated `.class()` calls accumulate
    div.title("A tooltip");   // sets the `title` attribute
    div.attribute("data-foo", "bar"); // any other attribute, by name
});
```

produces:

```html
<div id="main" class="card card--active" title="A tooltip" data-foo="bar"></div>
```

Two behaviors to know about:

- **`id` can only be set once per element.** Calling `.id(...)` twice on the
  same builder panics with `the "id" attribute must only be set once`.
- **`class` accumulates instead of overwriting.** Each `.class(...)` call
  appends to the existing value, space-separated. `.attribute(...)` with any
  other name, by contrast, *replaces* the previous value if called again.

**Attributes must be set before any child is added.** Once you call
`.text(...)` or open a nested element on a builder, that builder's attribute
phase is closed — calling an attribute method after that panics with `HTML
attributes must be declared before children`:

```rust
html.div(|div| {
    div.text("Hello");
    div.class("too-late"); // panics!
});
```

This mirrors how HTML actually works (attributes live inside the opening
tag), and the check happens at runtime, so make sure your attribute calls
come first in each closure.

### 5. Add text (escaping is automatic)

```rust
div.text("Hello, <world> & \"friends\"");
```

Text passed to `.text(...)` — and any attribute value — is HTML-escaped for
you (`&`, `<`, `>`, `"`, `'`). You never need to escape values by hand before
passing them in, and you don't need to worry about untrusted content
breaking out of a text node or an attribute.

`.text(...)` accepts anything implementing `std::fmt::Display`, so you can
pass numbers, formatted values, etc. directly:

```rust
div.text(42);
div.text(format!("{count} items"));
```

### 6. Nest elements

Element builders expose the same element-opening methods as `Html` itself,
so nesting just means calling `.div(...)` again from inside a closure:

```rust
html.div(|outer| {
    outer.class("outer");

    outer.div(|inner| {
        inner.class("inner");
        inner.text("Nested content");
    });

    outer.text("After the nested div");
});
```

### 7. Get the finished string, and do something with it

```rust
let output: String = html.finish();
```

`finish()` consumes the builder and hands you back a plain `String`. What you
do with it from there is entirely up to your application — htmlforge doesn't
know or care how it's used. A couple of common cases:

Write it to a file:

```rust
std::fs::write("page.html", html.finish())?;
```

Return it as the body of an HTTP response (works the same regardless of
which web framework you use — set the response body to the string and the
`Content-Type` header to `text/html; charset=utf-8`):

```rust
fn render_page() -> String {
    let mut html = Html::new();
    html.div(|div| {
        div.class("page");
        div.text("Hello from htmlforge");
    });
    html.finish()
}
```

## Current limitations

htmlforge is early — as of `0.1.0`:

- **Only `<div>` is implemented.** There's no `<p>`, `<span>`, `<a>`, lists,
  headings, or any other tag yet — just `Html` (the document root) and
  `Div`.
- **No fragments or conditionals built in.** Composition beyond nested
  closures (e.g. conditionally including an element, or reusable
  components) is up to you, using normal Rust control flow around the
  closures.
- **Invalid usage panics rather than returning a `Result`.** Setting `id`
  twice, or setting an attribute after starting a child, are programmer
  errors caught at runtime with a panic, not recoverable errors.

## Minimum supported Rust version

htmlforge uses the 2024 edition, so it requires a Rust toolchain that
supports it (1.85 or newer).

## License

Licensed under either of

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this project by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any additional
terms or conditions.
