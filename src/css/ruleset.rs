use crate::utils::get_indentation;
use crate::{CssDeclaration, CssProperty, CssSelector, CssValue};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Write};

#[derive(Debug, Clone)]
pub struct CssRuleset {
  selector: CssSelector,
  declarations: BTreeMap<CssProperty, CssDeclaration>,
}

impl CssRuleset {
  ///
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

  pub fn to_style(&self, offset: usize, indent: usize) -> String {
    let mut style = String::new();
    let _ = writeln!(&mut style, "{}{} {{", get_indentation(false, offset), self.selector);
    for declaration in self.declarations.values() {
      let _ = writeln!(&mut style, "{}{}", get_indentation(false, offset + indent), declaration);
    }
    let _ = writeln!(&mut style, "{}}}", get_indentation(false, offset));
    style
  }
}

impl Display for CssRuleset {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_style(0, 2))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{CssNumber, CssProperty, CssUnit, CssValue};

  #[test]
  fn display_should_work() {
    let ruleset = CssRuleset::new("p".into()).declaration(CssProperty::Width, CssValue::Num1(CssNumber::new(1.23, 2, CssUnit::Px)));
    assert_eq!(
      r"p {
  width: 1.23px;
}
",
      ruleset.to_string()
    )
  }
}
