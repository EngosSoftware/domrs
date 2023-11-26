use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SvgAttribute {
  Fill,
  Stroke,
  StrokeWidth,
}

impl Display for SvgAttribute {
  ///
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        SvgAttribute::Fill => "fill",
        SvgAttribute::Stroke => "stroke",
        SvgAttribute::StrokeWidth => "stroke-width",
      }
    )
  }
}
