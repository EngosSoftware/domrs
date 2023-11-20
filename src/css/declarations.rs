use crate::css::values::CssValue;
use crate::CssProperty;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct CssDeclaration {
  property: CssProperty,
  value: CssValue,
}

impl CssDeclaration {
  ///
  pub fn new(property: CssProperty, value: CssValue) -> Self {
    Self { property, value }
  }

  ///
  pub fn to_style(&self) -> String {
    format!("{}: {};", self.property, self.value)
  }
}

impl Display for CssDeclaration {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_style())
  }
}

#[cfg(test)]
mod tests {
  use crate::css::declarations::CssDeclaration;
  use crate::css::values::CssValue;
  use crate::{CssProperty, CssUnit};

  #[test]
  fn display_should_work() {
    assert_eq!(
      "width: 1.2346px;",
      CssDeclaration::new(CssProperty::Width, CssValue::Number((1.23456, 4, CssUnit::Px))).to_string()
    );
  }
}
