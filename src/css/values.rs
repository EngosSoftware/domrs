use crate::css::colors::CssColor;
use crate::utils::number_to_string;
use crate::{CssBorderStyle, CssBorderWidth, CssUnit};
use std::fmt;
use std::fmt::Display;

pub type CssNumber = (f64, usize, CssUnit);

#[derive(Copy, Clone)]
pub enum CssValue {
  Avoid,
  Border(CssBorderWidth, CssBorderStyle, CssColor),
  Center,
  Color(CssColor),
  Column,
  Flex,
  FlexStart,
  Grid,
  Integer(i64),
  Number(CssNumber),
  Number2(CssNumber, CssNumber),
  Relative,
  Row,
}

impl Display for CssValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssValue::Avoid => "avoid".to_string(),
        CssValue::Border(width, style, color) => format!("{} {} {}", width, style, color),
        CssValue::Center => "center".to_string(),
        CssValue::Color(color) => color.to_string(),
        CssValue::Column => "column".to_string(),
        CssValue::Flex => "flex".to_string(),
        CssValue::FlexStart => "flex-start".to_string(),
        CssValue::Grid => "grid".to_string(),
        CssValue::Integer(value) => format!("{}", value),
        CssValue::Number(number) => number_to_string(*number),
        CssValue::Number2(top_bottom, left_right) => format!("{} {}", number_to_string(*top_bottom), number_to_string(*left_right)),
        CssValue::Relative => "relative".to_string(),
        CssValue::Row => "row".to_string(),
      }
    )
  }
}

impl From<CssColor> for CssValue {
  fn from(value: CssColor) -> Self {
    CssValue::Color(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display_should_work() {
    assert_eq!("center", CssValue::Center.to_string());
    assert_eq!("1.234px", CssValue::Number((1.234123_f64, 3, CssUnit::Px)).to_string());
    assert_eq!("grid", CssValue::Grid.to_string());
  }
}
