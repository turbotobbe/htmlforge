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
