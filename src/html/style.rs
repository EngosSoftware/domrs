use crate::{CssDocument, HtmlElement, ToText, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};

/// A structure representing HTML `<style>` element.
#[derive(Debug, Clone)]
pub struct HtmlStyleElement {
  style: CssDocument,
  indent: usize,
}

impl HtmlStyleElement {
  pub fn new(style: CssDocument) -> Self {
    Self {
      style,
      indent: DEFAULT_CSS_INDENT,
    }
  }

  pub fn indent(mut self, indent: usize) -> Self {
    self.set_indent(indent);
    self
  }

  pub fn set_indent(&mut self, indent: usize) {
    self.indent = indent;
  }
}

impl From<HtmlStyleElement> for HtmlElement {
  fn from(value: HtmlStyleElement) -> Self {
    let mut style = HtmlElement::new("style");
    style.set_content(&value.style.to_text(DEFAULT_CSS_OFFSET, value.indent));
    style
  }
}
