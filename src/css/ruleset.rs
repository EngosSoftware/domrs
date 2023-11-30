use crate::common::get_indentation;
use crate::{CssDeclaration, CssProperty, CssSelector, CssValue, ToText, DEFAULT_CSS_INDENT, DEFAULT_CSS_OFFSET};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Write};

/// A structure representing CSS ruleset.
#[derive(Debug, Clone)]
pub struct CssRuleset {
  /// CSS selector.
  selector: CssSelector,
  /// CSS declarations.
  declarations: BTreeMap<CssProperty, CssDeclaration>,
}

impl CssRuleset {
  /// Creates a new ruleset with specified selector.
  pub fn new(selector: CssSelector) -> Self {
    Self {
      selector,
      declarations: BTreeMap::new(),
    }
  }

  pub fn declaration(mut self, property: impl Into<CssProperty>, value: impl Into<CssValue>) -> Self {
    self.add_declaration(property, value);
    self
  }

  pub fn add_declaration(&mut self, property: impl Into<CssProperty>, value: impl Into<CssValue>) {
    let declaration = CssDeclaration::new(property, value);
    self.declarations.insert(declaration.property, declaration);
  }

  pub fn declarations(mut self, declarations: &[CssDeclaration]) -> Self {
    self.add_declarations(declarations);
    self
  }

  pub fn add_declarations(&mut self, declarations: &[CssDeclaration]) {
    for declaration in declarations {
      self
        .declarations
        .insert(declaration.property, CssDeclaration::new(declaration.property, declaration.value.clone()));
    }
  }
}

impl ToText for CssRuleset {
  /// Converts [CssRuleset] to a textual representation using specified offset and indent.
  fn to_text(&self, offset: usize, indent: usize) -> String {
    let mut buffer = String::new();
    let _ = writeln!(&mut buffer, "{}{} {{", get_indentation(false, offset), self.selector);
    for declaration in self.declarations.values() {
      let _ = writeln!(&mut buffer, "{}{}", get_indentation(false, offset + indent), declaration);
    }
    let _ = writeln!(&mut buffer, "{}}}", get_indentation(false, offset));
    buffer
  }
}

impl Display for CssRuleset {
  /// Implements [Display] for [CssRuleset].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_text(DEFAULT_CSS_OFFSET, DEFAULT_CSS_INDENT))
  }
}
