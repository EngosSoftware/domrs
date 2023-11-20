use crate::utils::get_indentation;
use crate::{CssDeclaration, CssProperty, CssSelector, CssValue};
use std::fmt;
use std::fmt::{Display, Write};

pub struct CssRuleset {
  selector: CssSelector,
  declarations: Vec<CssDeclaration>,
}

impl CssRuleset {
  ///
  pub fn new(selector: CssSelector) -> Self {
    Self {
      selector,
      declarations: vec![],
    }
  }

  pub fn declaration(mut self, property: CssProperty, value: impl Into<CssValue>) -> Self {
    self.declarations.push(CssDeclaration::new(property, value.into()));
    self
  }

  ///
  pub fn to_style(&self, offset: usize, indent: usize) -> String {
    let mut style = String::new();

    let _ = writeln!(&mut style, "{}{} {{", get_indentation(false, offset), self.selector);
    for declaration in &self.declarations {
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
  use crate::{CssProperty, CssUnit, CssValue};

  #[test]
  fn display_should_work() {
    let ruleset = CssRuleset::new("p".into()).declaration(CssProperty::Width, CssValue::Number((1.23, 2, CssUnit::Px)));
    assert_eq!(
      r"p {
  width: 1.23px;
}
",
      ruleset.to_string()
    )
  }
}
