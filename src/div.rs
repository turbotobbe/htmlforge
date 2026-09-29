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
