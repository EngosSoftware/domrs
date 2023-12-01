use std::fmt;
use std::fmt::Display;

/// A structure representing the SVG number.
#[derive(Debug, Copy, Clone)]
pub struct SvgNumber(
  /// Number's value.
  f64,
  /// Number's precision.
  usize,
);

impl Display for SvgNumber {
  /// Implements [Display] for [SvgNumber].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{0:.1$}", self.0, self.1)
  }
}

impl SvgNumber {
  /// Creates a new SVG number based on provided value and precision.
  ///
  /// # Example
  ///
  /// ```
  /// use domrs::SvgNumber;
  ///
  /// assert_eq!("1", SvgNumber::new(1.1, 0).to_string());
  /// assert_eq!("1.17", SvgNumber::new(1.168, 2).to_string());
  /// ```
  pub fn new(value: f64, precision: usize) -> Self {
    Self(value, precision)
  }
}

impl From<u8> for SvgNumber {
  /// Creates [SvgNumber] from [u8].
  fn from(value: u8) -> Self {
    Self(value as f64, 0)
  }
}

impl From<i8> for SvgNumber {
  /// Creates [SvgNumber] from [i8].
  fn from(value: i8) -> Self {
    Self(value as f64, 0)
  }
}

impl From<u16> for SvgNumber {
  /// Creates [SvgNumber] from [u16].
  fn from(value: u16) -> Self {
    Self(value as f64, 0)
  }
}

impl From<i16> for SvgNumber {
  /// Creates [SvgNumber] from [i16].
  fn from(value: i16) -> Self {
    Self(value as f64, 0)
  }
}

impl From<u32> for SvgNumber {
  /// Creates [SvgNumber] from [u32].
  fn from(value: u32) -> Self {
    Self(value as f64, 0)
  }
}

impl From<i32> for SvgNumber {
  /// Creates [SvgNumber] from [i32].
  fn from(value: i32) -> Self {
    Self(value as f64, 0)
  }
}

impl From<u64> for SvgNumber {
  /// Creates [SvgNumber] from [u64].
  fn from(value: u64) -> Self {
    Self(value as f64, 0)
  }
}

impl From<i64> for SvgNumber {
  /// Creates [SvgNumber] from [i64].
  fn from(value: i64) -> Self {
    Self(value as f64, 0)
  }
}

impl From<f32> for SvgNumber {
  /// Creates [SvgNumber] from [f32].
  fn from(value: f32) -> Self {
    Self(value as f64, 0)
  }
}

impl From<f64> for SvgNumber {
  /// Creates [SvgNumber] from [f64].
  fn from(value: f64) -> Self {
    Self(value, 0)
  }
}
