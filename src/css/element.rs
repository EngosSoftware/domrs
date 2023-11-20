use crate::{CssGroup, CssRuleset, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};
use std::fmt;
use std::fmt::{Display, Write};

pub enum CssElement {
  Ruleset(CssRuleset),
  Group(CssGroup),
}

impl CssElement {
  /// Converts this [CssElement] into its text representation.
  pub fn to_style(&self, offset: usize, indent: usize) -> String {
    let mut style = String::new();
    match self {
      CssElement::Ruleset(ruleset) => {
        let _ = write!(&mut style, "{}", ruleset.to_style(offset, indent));
      }
      CssElement::Group(group) => {
        let _ = write!(&mut style, "{}", group.to_style(offset, indent));
      }
    }
    style
  }
}

impl Display for CssElement {
  /// Implements [Display] for [CssElement].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_style(DEFAULT_CSS_OFFSET, DEFAULT_CSS_INDENT))
  }
}
