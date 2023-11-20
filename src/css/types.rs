use crate::CssUnit;

pub enum CssType {
  Integer,
  Number,
  Dimension(CssUnit),
  Length(CssUnit),
  Angle(CssUnit),
  Time(CssUnit),
  Resolution(CssUnit),
  Percentage,
}
