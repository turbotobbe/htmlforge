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

    /// Like [`Html::new`], but indents nested elements and breaks each
    /// element and text node onto its own line.
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
