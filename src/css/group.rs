use crate::common::get_indentation;
use crate::{CssAtRule, CssRuleset, ToText, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};
use std::fmt;
use std::fmt::{Display, Write};

#[derive(Debug, Clone)]
pub struct CssGroup {
  rule: CssAtRule,
  query: String,
  rulesets: Vec<CssRuleset>,
}

impl CssGroup {
  pub fn media_print() -> Self {
    Self {
      rule: CssAtRule::Media,
      query: "print".to_string(),
      rulesets: vec![],
    }
  }

  pub fn media_screen() -> Self {
    Self {
      rule: CssAtRule::Media,
      query: "screen".to_string(),
      rulesets: vec![],
    }
  }

  pub fn media<T: ToString>(query: T) -> Self {
    Self {
      rule: CssAtRule::Media,
      query: query.to_string(),
      rulesets: vec![],
    }
  }

  /// Adds a new ruleset to this group.
  pub fn ruleset(mut self, ruleset: CssRuleset) -> Self {
    self.rulesets.push(ruleset);
    self
  }
}

impl ToText for CssGroup {
  /// Converts [CssGroup] to a textual representation using provided offset and indent.
  fn to_text(&self, offset: usize, indent: usize) -> String {
    let mut style = String::new();
    let _ = writeln!(&mut style, "{}{} {} {{", get_indentation(false, offset), self.rule, self.query);
    for ruleset in &self.rulesets {
      let _ = write!(&mut style, "{}", ruleset.to_text(offset + indent, indent));
    }
    let _ = writeln!(&mut style, "{}}}", get_indentation(false, offset));
    style
  }
}

impl Display for CssGroup {
  /// Implements [Display] for [CssGroup].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_text(DEFAULT_CSS_OFFSET, DEFAULT_CSS_INDENT))
  }
}
