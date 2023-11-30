use std::fmt;
use std::fmt::Display;

/// A structure representing a part of the CSS selector.
#[derive(Debug, Default, Clone)]
pub struct CssSelectorPart {
  /// Components that constitute a selector part.
  components: Vec<String>,
}

impl CssSelectorPart {
  /// Creates new selector part.
  pub fn new() -> Self {
    Self::default()
  }

  /// Adds a class component as a selector part.
  pub fn class<T: ToString>(mut self, class: T) -> Self {
    self.add_class(class);
    self
  }

  /// Adds a class component as a selector part.
  pub fn add_class<T: ToString>(&mut self, class: T) {
    self.components.push(format!(".{}", class.to_string()));
  }

  /// Adds an element component as a selector part.
  pub fn element<T: ToString>(mut self, element: T) -> Self {
    self.add_element(element);
    self
  }

  /// Adds an element component as a selector part.
  pub fn add_element<T: ToString>(&mut self, element: T) {
    self.components.push(element.to_string());
  }
}

impl Display for CssSelectorPart {
  /// Implements [Display] for [CssSelectorPart].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.components.to_vec().join(" "))
  }
}

impl From<&str> for CssSelectorPart {
  /// Creates [CssSelectorPart] from single string.
  fn from(value: &str) -> Self {
    Self {
      components: vec![value.to_string()],
    }
  }
}

impl From<&[&str]> for CssSelectorPart {
  /// Creates [CssSelectorPart] from a collection of strings.
  fn from(value: &[&str]) -> Self {
    Self {
      components: value.iter().map(|part| part.to_string()).collect(),
    }
  }
}

/// A structure representing the CSS selector.
#[derive(Default, Debug, Clone)]
pub struct CssSelector {
  parts: Vec<CssSelectorPart>,
}

impl CssSelector {
  /// Creates a new selector.
  pub fn new() -> Self {
    Self::default()
  }

  pub fn part(mut self, part: CssSelectorPart) -> Self {
    self.add_part(part);
    self
  }

  pub fn add_part(&mut self, part: CssSelectorPart) {
    self.parts.push(part);
  }

  pub fn class<T: ToString>(mut self, class: T) -> Self {
    self.add_class(class);
    self
  }

  pub fn add_class<T: ToString>(&mut self, class: T) {
    self.parts.push(CssSelectorPart::new().class(class));
  }

  pub fn element<T: ToString>(mut self, element: T) -> Self {
    self.add_element(element);
    self
  }

  pub fn add_element<T: ToString>(&mut self, element: T) {
    self.parts.push(CssSelectorPart::new().element(element));
  }
}

impl Display for CssSelector {
  /// Implements [Display] for [CssSelector].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.parts.iter().map(|part| part.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl From<&str> for CssSelector {
  /// Creates [CssSelector] from string.
  fn from(value: &str) -> Self {
    Self {
      parts: vec![CssSelectorPart::from(value)],
    }
  }
}

impl From<&[&str]> for CssSelector {
  /// Creates [CssSelector] from a collection of strings.
  fn from(value: &[&str]) -> Self {
    Self {
      parts: value.iter().map(|p| CssSelectorPart::from(*p)).collect(),
    }
  }
}

impl From<CssSelectorPart> for CssSelector {
  /// Creates [CssSelector] from [CssSelectorPart].
  fn from(value: CssSelectorPart) -> Self {
    Self { parts: vec![value] }
  }
}

impl From<&[CssSelectorPart]> for CssSelector {
  /// Creates [CssSelector] from a slice of [CssSelectorPart].
  fn from(value: &[CssSelectorPart]) -> Self {
    Self { parts: value.to_vec() }
  }
}
