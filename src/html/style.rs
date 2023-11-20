use crate::{CssDocument, HtmlElement, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};

/// Structure representing the HTML `<style>` element.
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

  pub fn new_indent(style: CssDocument, indent: usize) -> Self {
    Self { style, indent }
  }
}

impl From<HtmlStyleElement> for HtmlElement {
  fn from(value: HtmlStyleElement) -> Self {
    let mut style = HtmlElement::new("style");
    style.set_content(value.style.to_style(DEFAULT_CSS_OFFSET, value.indent));
    style
  }
}
