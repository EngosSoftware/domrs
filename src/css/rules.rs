use std::fmt;
use std::fmt::Display;

/// An enumeration representing a CSS `@` rule.
#[derive(Debug, Clone)]
pub enum CssAtRule {
  /// `@media` rule.
  Media,
}

impl Display for CssAtRule {
  /// Implements [Display] for [CssAtRule].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssAtRule::Media => "@media",
      }
    )
  }
}
