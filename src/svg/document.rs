use crate::HtmlElement;

pub const DEFAULT_SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

#[derive(Default, Debug, Clone)]
pub struct SvgDocument {
  width: Option<String>,
  height: Option<String>,
}

impl SvgDocument {
  /// Adds width.
  pub fn with_width(mut self, width: String) -> Self {
    self.width = width.into();
    self
  }

  /// Adds height.
  pub fn with_height(mut self, height: String) -> Self {
    self.height = height.into();
    self
  }
}

impl From<SvgDocument> for HtmlElement {
  fn from(value: SvgDocument) -> Self {
    let mut svg = HtmlElement::new("svg");
    if let Some(width) = value.width {
      svg.set_attr("width", width);
    }
    if let Some(height) = value.height {
      svg.set_attr("height", height);
    }
    svg.set_attr("xmlns", DEFAULT_SVG_NAMESPACE);
    svg
  }
}
