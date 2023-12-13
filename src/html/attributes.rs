use std::fmt;
use std::fmt::Display;

/// A structure representing HTML attribute.
#[derive(Debug, Clone)]
pub struct HtmlAttribute {
  name: String,
  value: String,
}

impl Display for HtmlAttribute {
  /// Implements [Display] for [HtmlAttribute].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, r#" {}="{}""#, self.name, self.value)
  }
}

impl HtmlAttribute {
  /// Creates and attribute with provided name and value.
  pub fn new<N, V>(name: N, value: V) -> Self
  where
    N: ToString,
    V: ToString,
  {
    Self {
      name: name.to_string(),
      value: value.to_string(),
    }
  }
}
