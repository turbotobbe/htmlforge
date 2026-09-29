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

    /// Starts a new child (an element or a text node): in pretty mode, breaks
    /// onto its own line and indents to the current depth.
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
