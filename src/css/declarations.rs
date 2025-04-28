use crate::css::values::CssValue;
use crate::CssProperty;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct CssDeclaration {
  pub(crate) property: CssProperty,
  pub(crate) value: CssValue,
}

impl CssDeclaration {
  /// Creates a new CSS declaration.
  pub fn new(property: impl Into<CssProperty>, value: impl Into<CssValue>) -> Self {
    Self {
      property: property.into(),
      value: value.into(),
    }
  }
}

impl Display for CssDeclaration {
  /// Implements [Display] for [CssDeclaration].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}: {};", self.property, self.value)
  }
}

#[cfg(test)]
mod tests {
  use crate::css::declarations::CssDeclaration;
  use crate::css::values::CssValue;
  use crate::{CssNumber, CssProperty, CssUnit};

  #[test]
  fn display_should_work() {
    assert_eq!(
      "width: 1.2346px;",
      CssDeclaration::new(CssProperty::Width, CssValue::Num1(CssNumber::new(1.23456, 4, CssUnit::Px))).to_string()
    );
  }
}
