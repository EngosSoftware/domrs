use crate::{CssGroup, CssRuleset, ToText, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};
use std::fmt;
use std::fmt::{Display, Write};

/// A structure representing CSS element.
#[derive(Debug, Clone)]
pub enum CssElement {
  /// CSS ruleset.
  Ruleset(CssRuleset),
  /// CSS group.
  Group(CssGroup),
}

impl ToText for CssElement {
  /// Converts [CssElement] to a textual representation using provided offset and indent.
  fn to_text(&self, offset: usize, indent: usize) -> String {
    let mut style = String::new();
    match self {
      CssElement::Ruleset(ruleset) => {
        let _ = write!(&mut style, "{}", ruleset.to_text(offset, indent));
      }
      CssElement::Group(group) => {
        let _ = write!(&mut style, "{}", group.to_text(offset, indent));
      }
    }
    style
  }
}

impl Display for CssElement {
  /// Implements [Display] for [CssElement].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_text(DEFAULT_CSS_OFFSET, DEFAULT_CSS_INDENT))
  }
}
