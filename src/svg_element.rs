use crate::HtmlElement;

const SVG_XMLNS: &str = "http://www.w3.org/2000/svg";

#[derive(Default, Debug, Clone)]
pub struct SvgElement {
  width: Option<String>,
  height: Option<String>,
}

impl SvgElement {
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

impl From<SvgElement> for HtmlElement {
  fn from(value: SvgElement) -> Self {
    let mut svg = HtmlElement::new("svg");
    if let Some(width) = value.width {
      svg.set_attr("width", width);
    }
    if let Some(height) = value.height {
      svg.set_attr("height", height);
    }
    svg.set_attr("xmlns", SVG_XMLNS);
    svg
  }
}
