use std::fmt;
use std::fmt::Display;

/// An enumeration representing attributes used in SVG elements.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SvgAttribute {
  /// The [`fill`] attribute used in SVG elements.
  ///
  /// [`fill`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill
  Fill,
  /// The [`stroke`] attribute used in SVG elements.
  ///
  /// [`stroke`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke
  Stroke,
  /// The [`stroke-width`] attribute used in SVG elements.
  ///
  /// [`stroke-width`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-width
  StrokeWidth,
}

impl Display for SvgAttribute {
  /// Implements [Display] for [SvgAttribute].
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
