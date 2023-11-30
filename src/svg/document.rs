use crate::common::ToText;
use crate::svg::{DEFAULT_SVG_INDENT, DEFAULT_SVG_OFFSET};
use crate::HtmlElement;
use std::fmt;
use std::fmt::Display;

/// Default namespace for SVG document.
pub const DEFAULT_SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// A structure representing SVG document.
#[derive(Debug, Default, Clone)]
pub struct SvgDocument {
  /// The displayed width of the rectangular viewport (not the width of its coordinate system).
  width: Option<String>,
  /// The displayed height of the rectangular viewport (not the height of its coordinate system).
  height: Option<String>,
}

impl SvgDocument {
  /// Creates an empty SVG document.
  pub fn new() -> Self {
    Default::default()
  }

  /// Adds the displayed width of the rectangular viewport to SVG document.
  pub fn width(mut self, width: String) -> Self {
    self.width = width.into();
    self
  }

  /// Adds the displayed height of the rectangular viewport to SVG document.
  pub fn height(mut self, height: String) -> Self {
    self.height = height.into();
    self
  }
}

impl ToText for SvgDocument {
  /// Converts [SvgDocument] to a textual representation with
  /// provided offset and indent.
  fn to_text(&self, offset: usize, indent: usize) -> String {
    let svg: HtmlElement = (*self).clone().into();
    svg.to_text(offset, indent)
  }
}

impl Display for SvgDocument {
  /// Implements [Display] for [SvgDocument].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_text(DEFAULT_SVG_OFFSET, DEFAULT_SVG_INDENT))
  }
}

impl From<SvgDocument> for HtmlElement {
  /// Creates [HtmlElement] from [SvgDocument].
  fn from(value: SvgDocument) -> Self {
    let mut svg = HtmlElement::new("svg");
    if let Some(width) = value.width {
      svg.set_attribute("width", &width);
    }
    if let Some(height) = value.height {
      svg.set_attribute("height", &height);
    }
    svg.set_attribute("xmlns", DEFAULT_SVG_NAMESPACE);
    svg
  }
}
