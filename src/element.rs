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

    /// Sets an attribute, replacing any previous value set under the same name.
    pub(crate) fn set_attribute(&mut self, name: &str, value: impl Display) {
        let value = value.to_string();
        match self.attributes.iter_mut().find(|(n, _)| n == name) {
            Some(existing) => existing.1 = value,
            None => self.attributes.push((name.to_string(), value)),
        }
    }

    /// Sets an attribute that may only be set once (e.g. `id`). Panics if the
    /// attribute has already been set.
    pub(crate) fn set_attribute_once(&mut self, name: &str, value: impl Display) {
        assert!(
            !self.attributes.iter().any(|(n, _)| n == name),
            "the \"{name}\" attribute must only be set once"
        );
        self.set_attribute(name, value);
    }

    /// Sets an attribute, appending to any previous value set under the same
    /// name (separated by `separator`) instead of replacing it. Used for
    /// attributes like `class` where repeated calls should accumulate.
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
