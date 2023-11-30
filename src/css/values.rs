use crate::{CssBorder, CssColor, CssFontFamily, CssFontStyle, CssNumber};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum CssValue {
  Avoid,
  Block,
  Border(CssBorder),
  Center,
  Color(CssColor),
  Column,
  Flex,
  FlexStart,
  FontFamily(CssFontFamily),
  FontStyle(CssFontStyle),
  Grid,
  Hidden,
  InlineBlock,
  Integer(i64),
  Left,
  None,
  Num1(CssNumber),
  Num2(CssNumber, CssNumber),
  Num3(CssNumber, CssNumber, CssNumber),
  Num4(CssNumber, CssNumber, CssNumber, CssNumber),
  Relative,
  Right,
  Row,
  Start,
  Unset,
  Zero,
}

impl Display for CssValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssValue::Avoid => "avoid".to_string(),
        CssValue::Block => "block".to_string(),
        CssValue::Border(border) => border.to_string(),
        CssValue::Center => "center".to_string(),
        CssValue::Color(color) => color.to_string(),
        CssValue::Column => "column".to_string(),
        CssValue::Flex => "flex".to_string(),
        CssValue::FlexStart => "flex-start".to_string(),
        CssValue::FontFamily(family) => family.to_string(),
        CssValue::FontStyle(style) => style.to_string(),
        CssValue::Grid => "grid".to_string(),
        CssValue::Hidden => "hidden".to_string(),
        CssValue::InlineBlock => "inline-block".to_string(),
        CssValue::Integer(value) => format!("{}", value),
        CssValue::Left => "left".to_string(),
        CssValue::None => "none".to_string(),
        CssValue::Num1(n1) => n1.to_string(),
        CssValue::Num2(n1, n2) => format!("{} {}", n1, n2),
        CssValue::Num3(n1, n2, n3) => format!("{} {} {}", n1, n2, n3),
        CssValue::Num4(n1, n2, n3, n4) => format!("{} {} {} {}", n1, n2, n3, n4),
        CssValue::Relative => "relative".to_string(),
        CssValue::Right => "right".to_string(),
        CssValue::Row => "row".to_string(),
        CssValue::Start => "start".to_string(),
        CssValue::Unset => "unset".to_string(),
        CssValue::Zero => "0".to_string(),
      }
    )
  }
}

impl From<CssBorder> for CssValue {
  fn from(value: CssBorder) -> Self {
    CssValue::Border(value)
  }
}

impl From<CssColor> for CssValue {
  fn from(value: CssColor) -> Self {
    CssValue::Color(value)
  }
}

impl From<CssFontFamily> for CssValue {
  fn from(value: CssFontFamily) -> Self {
    CssValue::FontFamily(value)
  }
}

impl From<CssFontStyle> for CssValue {
  fn from(value: CssFontStyle) -> Self {
    CssValue::FontStyle(value)
  }
}

impl From<CssNumber> for CssValue {
  fn from(value: CssNumber) -> Self {
    CssValue::Num1(value)
  }
}

impl From<(CssNumber, CssNumber)> for CssValue {
  fn from((n1, n2): (CssNumber, CssNumber)) -> Self {
    CssValue::Num2(n1, n2)
  }
}

impl From<(CssNumber, CssNumber, CssNumber)> for CssValue {
  fn from((n1, n2, n3): (CssNumber, CssNumber, CssNumber)) -> Self {
    CssValue::Num3(n1, n2, n3)
  }
}

impl From<(CssNumber, CssNumber, CssNumber, CssNumber)> for CssValue {
  fn from((n1, n2, n3, n4): (CssNumber, CssNumber, CssNumber, CssNumber)) -> Self {
    CssValue::Num4(n1, n2, n3, n4)
  }
}

impl From<u8> for CssValue {
  fn from(value: u8) -> Self {
    CssValue::Integer(value as i64)
  }
}

impl From<i8> for CssValue {
  fn from(value: i8) -> Self {
    CssValue::Integer(value as i64)
  }
}

impl From<u16> for CssValue {
  fn from(value: u16) -> Self {
    CssValue::Integer(value as i64)
  }
}

impl From<i16> for CssValue {
  fn from(value: i16) -> Self {
    CssValue::Integer(value as i64)
  }
}

impl From<u32> for CssValue {
  fn from(value: u32) -> Self {
    CssValue::Integer(value as i64)
  }
}

impl From<i32> for CssValue {
  fn from(value: i32) -> Self {
    CssValue::Integer(value as i64)
  }
}

impl From<u64> for CssValue {
  fn from(value: u64) -> Self {
    CssValue::Integer(value as i64)
  }
}

impl From<i64> for CssValue {
  fn from(value: i64) -> Self {
    CssValue::Integer(value)
  }
}
