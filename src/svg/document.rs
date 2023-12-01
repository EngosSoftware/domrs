use crate::common::ToText;
use crate::svg::{DEFAULT_SVG_INDENT, DEFAULT_SVG_OFFSET};
use crate::{HtmlElement, SvgNumber};
use std::fmt;
use std::fmt::Display;

/// Default namespace for SVG document.
pub const DEFAULT_SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// A structure representing the outermost [`svg`] element of SVG document.
///
/// [`svg`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg
#[derive(Debug, Default, Clone)]
pub struct SvgDocument {
  /// CSS document's [`namespace`].
  ///
  /// [`namespace`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Namespaces_Crash_Course
  namespace: Option<String>,
  /// The [`position and dimension`], in user space, of an SVG viewport.
  ///
  /// [`position and dimension`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox
  view_box: Option<(SvgNumber, SvgNumber, SvgNumber, SvgNumber)>,
  /// The displayed [`width`] of the rectangular viewport (not the width of its coordinate system).
  ///
  /// [`width`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/width
  width: Option<SvgNumber>,
  /// The displayed [`height`] of the rectangular viewport (not the height of its coordinate system).
  ///
  /// [`height`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height
  height: Option<SvgNumber>,
}

impl SvgDocument {
  /// Creates an empty SVG document.
  ///
  /// # Example
  ///
  /// ```
  /// use domrs::SvgDocument;
  ///
  /// assert_eq!("<svg/>", SvgDocument::new().to_string());
  /// ```
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the default SVG [`namespace`].
  ///
  /// [`namespace`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Namespaces_Crash_Course
  pub fn default_namespace(mut self) -> Self {
    self.set_default_namespace();
    self
  }

  /// Sets the default SVG [`namespace`].
  ///
  /// [`namespace`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Namespaces_Crash_Course
  pub fn set_default_namespace(&mut self) {
    self.namespace = DEFAULT_SVG_NAMESPACE.to_string().into();
  }

  /// Sets custom SVG [`namespace`].
  ///
  /// [`namespace`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Namespaces_Crash_Course
  pub fn namespace<T: ToString>(mut self, namespace: T) -> Self {
    self.set_namespace(namespace);
    self
  }

  /// Sets custom SVG [`namespace`].
  ///
  /// [`namespace`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Namespaces_Crash_Course
  pub fn set_namespace<T: ToString>(&mut self, namespace: T) {
    self.namespace = namespace.to_string().into();
  }

  /// Sets the [`position and dimension`], in user space, of an SVG viewport.
  ///
  /// [`position and dimension`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox
  pub fn view_box<T: Into<SvgNumber>>(mut self, min_x: T, min_y: T, width: T, height: T) -> Self {
    self.set_view_box(min_x, min_y, width, height);
    self
  }

  /// Sets the [`position and dimension`], in user space, of an SVG viewport.
  ///
  /// [`position and dimension`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox
  pub fn set_view_box<T: Into<SvgNumber>>(&mut self, min_x: T, min_y: T, width: T, height: T) {
    self.view_box = Some((min_x.into(), min_y.into(), width.into(), height.into()));
  }

  /// Sets the displayed [`width`] attribute.
  ///
  /// [`width`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/width
  pub fn width<T: Into<SvgNumber>>(mut self, width: T) -> Self {
    self.set_width(width);
    self
  }

  /// Sets the displayed [`width`] attribute.
  ///
  /// [`width`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/width
  pub fn set_width<T: Into<SvgNumber>>(&mut self, width: T) {
    self.width = Some(width.into());
  }

  /// Sets the displayed [`height`] attribute.
  ///
  /// [`height`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height
  pub fn height<T: Into<SvgNumber>>(mut self, height: T) -> Self {
    self.set_height(height);
    self
  }

  /// Sets the displayed [`height`] attribute.
  ///
  /// [`height`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height
  pub fn set_height<T: Into<SvgNumber>>(&mut self, height: T) {
    self.height = Some(height.into());
  }
}

impl ToText for SvgDocument {
  /// Converts [SvgDocument] to a textual representation with provided offset and indent.
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
      svg.set_attribute("width", &width.to_string());
    }
    if let Some(height) = value.height {
      svg.set_attribute("height", &height.to_string());
    }
    if let Some(namespace) = value.namespace {
      svg.set_attribute("xmlns", &namespace);
    }
    if let Some((min_x, min_y, width, height)) = value.view_box {
      svg.set_attribute("viewBox", &format!("{} {} {} {}", min_x, min_y, width, height));
    }
    svg
  }
}
