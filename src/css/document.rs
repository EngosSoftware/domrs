use crate::{CssElement, CssGroup, CssRuleset, ToText, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};
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
    Self::default()
  }

  /// Adds a new element to CSS document.
  pub fn element(mut self, element: CssElement) -> Self {
    self.add_element(element);
    self
  }

  /// Adds a new element to CSS document.
  pub fn add_element(&mut self, element: CssElement) {
    self.elements.push(element);
  }

  /// Adds a new ruleset to CSS document.
  pub fn ruleset(mut self, ruleset: CssRuleset) -> Self {
    self.add_ruleset(ruleset);
    self
  }

  /// Adds a new ruleset to CSS document.
  pub fn add_ruleset(&mut self, ruleset: CssRuleset) {
    self.elements.push(CssElement::Ruleset(ruleset));
  }

  /// Adds a new group to CSS document.
  pub fn group(mut self, group: CssGroup) -> Self {
    self.add_group(group);
    self
  }

  /// Adds a new group to CSS document.
  pub fn add_group(&mut self, group: CssGroup) {
    self.elements.push(CssElement::Group(group));
  }
}

impl ToText for CssDocument {
  /// Converts [CssDocument] to a textual representation using provided offset and indent.
  fn to_text(&self, offset: usize, indent: usize) -> String {
    let mut buffer = String::new();
    for element in &self.elements {
      let _ = write!(&mut buffer, "{}", element.to_text(offset, indent));
    }
    buffer
  }
}

impl Display for CssDocument {
  /// Implements [Display] for [CssDocument].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_text(DEFAULT_CSS_OFFSET, DEFAULT_CSS_INDENT))
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
