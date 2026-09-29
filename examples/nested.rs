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
