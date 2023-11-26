use crate::{CssColor, CssNumber};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct CssBorder(CssNumber, CssBorderStyle, CssColor);

impl CssBorder {
  pub fn new(width: CssNumber, style: CssBorderStyle, color: CssColor) -> Self {
    Self(width, style, color)
  }
}

impl Display for CssBorder {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.0, self.1, self.2)
  }
}

#[derive(Debug, Copy, Clone)]
pub enum CssBorderStyle {
  Dotted,
  Dashed,
  Solid,
  Double,
  Groove,
  Ridge,
  Inset,
  Outset,
  None,
  Hidden,
}

impl Display for CssBorderStyle {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssBorderStyle::Dotted => "dotted",
        CssBorderStyle::Dashed => "dashed",
        CssBorderStyle::Solid => "solid",
        CssBorderStyle::Double => "double",
        CssBorderStyle::Groove => "groove",
        CssBorderStyle::Ridge => "ridge",
        CssBorderStyle::Inset => "inset",
        CssBorderStyle::Outset => "outset",
        CssBorderStyle::None => "none",
        CssBorderStyle::Hidden => "hidden",
      }
    )
  }
}

#[macro_export]
macro_rules! border {
  ($width:expr, $style:expr, $color:expr) => {
    CssBorder::new($width, $style, $color)
  };
}
