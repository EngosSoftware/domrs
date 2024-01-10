use crate::CssUnit;
use std::fmt;
use std::fmt::Display;

/// A structure representing the CSS number.
#[derive(Debug, Copy, Clone)]
pub struct CssNumber(f64, usize, CssUnit);

impl Display for CssNumber {
  /// Implements [Display] for [CssNumber].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.2 == CssUnit::Auto {
      write!(f, "auto")
    } else if self.0 == 0.0 {
      write!(f, "0")
    } else {
      write!(f, "{0:.1$}{2}", self.0, self.1, self.2)
    }
  }
}

impl CssNumber {
  /// Creates a new number based on provided value, precision and unit.
  pub fn new(value: f64, precision: usize, unit: CssUnit) -> Self {
    Self(value, precision, unit)
  }
}

#[macro_export]
macro_rules! css_num {
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
