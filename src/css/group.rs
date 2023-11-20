use crate::utils::get_indentation;
use crate::{CssRuleset, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};
use std::fmt;
use std::fmt::{Display, Write};

#[derive(Debug, Clone)]
pub enum CssAtRule {
  Media,
}

impl Display for CssAtRule {
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

  pub fn new_media(query: &str) -> Self {
    Self {
      rule: CssAtRule::Media,
      query: query.to_string(),
      rulesets: vec![],
    }
  }

  /// Adds a new [CssRuleset] to this group.
  pub fn ruleset(mut self, ruleset: CssRuleset) -> Self {
    self.rulesets.push(ruleset);
    self
  }

  pub fn to_style(&self, offset: usize, indent: usize) -> String {
    let mut style = String::new();
    let _ = writeln!(&mut style, "{}{} {} {{", get_indentation(false, offset), self.rule, self.query);
    for ruleset in &self.rulesets {
      let _ = write!(&mut style, "{}", ruleset.to_style(offset + indent, indent));
    }
    let _ = writeln!(&mut style, "{}}}", get_indentation(false, offset));
    style
  }
}

impl Display for CssGroup {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_style(DEFAULT_CSS_OFFSET, DEFAULT_CSS_INDENT))
  }
}
