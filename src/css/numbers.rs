use crate::CssUnit;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct CssNumber(f64, usize, CssUnit);

impl CssNumber {
  ///
  pub fn new(value: f64, precision: usize, unit: CssUnit) -> Self {
    Self(value, precision, unit)
  }
}

impl Display for CssNumber {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self.2 {
        unit @ CssUnit::Auto => unit.to_string(),
        _ => format!("{0:.1$}{2}", self.0, self.1, self.2),
      }
    )
  }
}

#[macro_export]
macro_rules! num {
  ($value:expr, $precision:expr, $unit:expr) => {
    CssNumber::new($value, $precision, $unit)
  };
}

#[macro_export]
macro_rules! px {
  ($value:expr) => {
    CssNumber::new($value as f64, 0, CssUnit::Px)
  };
}

#[macro_export]
macro_rules! pt {
  ($value:expr, $precision:expr) => {
    CssNumber::new($value as f64, $precision, CssUnit::Pt)
  };
  ($value:expr) => {
    CssNumber::new($value as f64, 0, CssUnit::Pt)
  };
}

#[macro_export]
macro_rules! em {
  ($value:expr, $precision:expr) => {
    CssNumber::new($value as f64, $precision, CssUnit::Em)
  };
  ($value:expr) => {
    CssNumber::new($value as f64, 0, CssUnit::Em)
  };
}

#[macro_export]
macro_rules! perc {
  ($value:expr, $precision:expr) => {
    CssNumber::new($value as f64, $precision, CssUnit::Perc)
  };
  ($value:expr) => {
    CssNumber::new($value as f64, 0, CssUnit::Perc)
  };
}

#[macro_export]
macro_rules! zero {
  () => {
    CssNumber::new(0.0, 0, CssUnit::None)
  };
}

#[macro_export]
macro_rules! auto {
  () => {
    CssNumber::new(0.0, 0, CssUnit::Auto)
  };
}
