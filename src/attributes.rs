use std::fmt::Display;

use crate::element::Element;

pub trait Attributes<'a> {
    fn element(&mut self) -> &mut Element<'a>;

    fn id(&mut self, value: impl Display) {
        let element = self.element();
        element.assert_attributes_allowed();
        element.set_attribute_once("id", value);
    }

    fn class(&mut self, value: impl Display) {
        let element = self.element();
        element.assert_attributes_allowed();
        element.append_attribute("class", value, " ");
    }

    fn title(&mut self, value: impl Display) {
        self.attribute("title", value);
    }

    fn attribute(&mut self, name: &str, value: impl Display) {
        let element = self.element();
        element.assert_attributes_allowed();
        element.set_attribute(name, value);
    }
}
