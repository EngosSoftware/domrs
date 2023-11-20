use crate::css::values::CssNumber;
use crate::utils::number_to_string;
use crate::CssUnit;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct CssBorderWidth(CssNumber);

impl CssBorderWidth {
  pub fn new(value: f64, precision: usize, unit: CssUnit) -> Self {
    Self((value, precision, unit))
  }
}

impl Display for CssBorderWidth {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", number_to_string(self.0))
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
