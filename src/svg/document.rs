use crate::common::ToText;
use crate::svg::{DEFAULT_SVG_INDENT, DEFAULT_SVG_OFFSET};
use crate::HtmlElement;
use std::fmt;
use std::fmt::Display;

pub const DEFAULT_SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

#[derive(Debug, Default, Clone)]
pub struct SvgDocument {
  width: Option<String>,
  height: Option<String>,
}

impl SvgDocument {
  /// Creates an empty SVG document.
  pub fn new() -> Self {
    Default::default()
  }

  /// Adds width to SVG document.
  pub fn width(mut self, width: String) -> Self {
    self.width = width.into();
    self
  }

  /// Adds height to SVG document.
  pub fn height(mut self, height: String) -> Self {
    self.height = height.into();
    self
  }
}

impl ToText for SvgDocument {
  /// Converts [SvgDocument] into its text representation.
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
