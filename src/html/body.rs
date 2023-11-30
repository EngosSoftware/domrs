use crate::HtmlElement;
use std::fmt;
use std::fmt::Display;

/// A structure representing HTML `<body>` element.
#[derive(Debug, Default, Clone)]
pub struct HtmlBodyElement {
  children: Vec<HtmlElement>,
}

impl HtmlBodyElement {
  /// Creates new body element.
  ///
  /// # Example
  ///
  /// ```
  /// # use domrs::HtmlBodyElement;
  /// let body = HtmlBodyElement::new();
  /// assert_eq!("<body/>", body.to_string());
  /// ```
  pub fn new() -> Self {
    Self::default()
  }

  /// Adds child element into body element.
  ///
  /// ```
  /// # use domrs::{HtmlBodyElement, HtmlElement};
  /// let h1 = HtmlElement::h1("hello");
  /// let body = HtmlBodyElement::new().child(h1);
  /// assert_eq!("<body>\n  <h1>hello</h1>\n</body>", body.to_string());
  /// ```
  pub fn child(mut self, child: impl Into<HtmlElement>) -> Self {
    self.add_child(child);
    self
  }

  /// Adds child element into body element.
  ///
  /// ```
  /// # use domrs::{HtmlBodyElement, HtmlElement};
  /// let h1 = HtmlElement::h1("hello");
  /// let mut body = HtmlBodyElement::new();
  /// body.add_child(h1);
  /// assert_eq!("<body>\n  <h1>hello</h1>\n</body>", body.to_string());
  /// ```
  pub fn add_child(&mut self, child: impl Into<HtmlElement>) {
    self.children.push(child.into());
  }
}

impl Display for HtmlBodyElement {
  /// Implements [Display] for [HtmlBodyElement].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", <&HtmlBodyElement as Into<HtmlElement>>::into(self))
  }
}

impl From<HtmlBodyElement> for HtmlElement {
  /// Converts [HtmlBodyElement] into [HtmlElement].
  fn from(value: HtmlBodyElement) -> Self {
    let mut body = HtmlElement::new("body").no_indent();
    for child in value.children {
      body.add_child(child);
    }
    body
  }
}

impl From<&HtmlBodyElement> for HtmlElement {
  /// Converts a reference to [HtmlBodyElement] into [HtmlElement].
  fn from(value: &HtmlBodyElement) -> Self {
    Self::from(value.clone())
  }
}
