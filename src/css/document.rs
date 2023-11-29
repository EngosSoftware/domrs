use crate::{CssElement, CssGroup, CssRuleset, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};
use std::fmt;
use std::fmt::{Display, Write};
use std::ops::Add;

#[derive(Debug, Default, Clone)]
pub struct CssDocument {
  elements: Vec<CssElement>,
}

impl CssDocument {
  /// Creates an empty CSS document.
  pub fn new() -> Self {
    Default::default()
  }

  /// Adds a new element to CSS document.
  pub fn add_element(&mut self, element: CssElement) {
    self.elements.push(element);
  }

  /// Adds a new ruleset to CSS document.
  pub fn ruleset(mut self, ruleset: CssRuleset) -> Self {
    self.elements.push(CssElement::Ruleset(ruleset));
    self
  }

  /// Adds a new group to CSS document.
  pub fn group(mut self, group: CssGroup) -> Self {
    self.elements.push(CssElement::Group(group));
    self
  }

  /// Converts this [CssDocument] into its text representation.
  pub fn to_style(&self, offset: usize, indent: usize) -> String {
    let mut style = String::new();
    for element in &self.elements {
      let _ = write!(&mut style, "{}", element.to_style(offset, indent));
    }
    style
  }
}

impl Display for CssDocument {
  /// Implements [Display] for [CssDocument].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_style(DEFAULT_CSS_OFFSET, DEFAULT_CSS_INDENT))
  }
}

impl Add for CssDocument {
  type Output = CssDocument;

  fn add(self, rhs: Self) -> Self::Output {
    let mut out = self.clone();
    out.elements.append(&mut rhs.elements.clone());
    out
  }
}
