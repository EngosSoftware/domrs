use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum CssUnit {
  /// No unit.
  None,
  /// centimeters, 1cm = 37.8px = 25.2/64in.
  Cm,
  /// Millimeters, 1mm = 1/10th of 1cm
  Mm,
  /// Quarter-millimeters, 1Q = 1/40th of 1cm
  Q,
  /// Inches, 1in = 2.54cm = 96px
  In,
  /// Picas, 1pc = 1/6th of 1in
  Pc,
  /// Points, 1pt = 1/72nd of 1in
  Pt,
  /// Pixels, 1px = 1/96th of 1in
  Px,
  /// Font size of the parent, in the case of typographical properties like font-size, and font size of the element itself, in the case of other properties like width.
  Em,
  /// x-height of the element's font.
  Ex,
  /// The advance measure (width) of the glyph "0" of the element's font.
  Ch,
  /// Font size of the root element.
  Rem,
  /// Line height of the element.
  Lh,
  /// Line height of the root element.
  Rlh,
  /// 1% of the viewport's width.
  Vw,
  /// 1% of the viewport's height.
  Vh,
  /// 1% of the viewport's smaller dimension.
  Vmin,
  /// 1% of the viewport's larger dimension.
  Vmax,
  /// 1% of the size of the initial containing block in the direction of the root element's block axis.
  Vb,
  /// 1% of the size of the initial containing block in the direction of the root element's inline axis.
  Vi,
  /// 1% of the small viewport's width.
  Svw,
  /// 1% of the small viewport's height.
  Svh,
  /// 1% of the large viewport's width.
  Lvw,
  /// 1% of the large viewport's height.
  Lvh,
  /// 1% of the dynamic viewport's width.
  Dvw,
  /// 1% of the dynamic viewport's height.
  Dvh,
}

impl Display for CssUnit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssUnit::None => "",
        CssUnit::Cm => "cm",
        CssUnit::Mm => "mm",
        CssUnit::Q => "Q",
        CssUnit::In => "in",
        CssUnit::Pc => "pc",
        CssUnit::Pt => "pt",
        CssUnit::Px => "px",
        CssUnit::Em => "em",
        CssUnit::Ex => "ex",
        CssUnit::Ch => "ch",
        CssUnit::Rem => "rem",
        CssUnit::Lh => "lh",
        CssUnit::Rlh => "rlh",
        CssUnit::Vw => "vw",
        CssUnit::Vh => "vh",
        CssUnit::Vmin => "vmin",
        CssUnit::Vmax => "vmax",
        CssUnit::Vb => "vb",
        CssUnit::Vi => "vi",
        CssUnit::Svw => "svw",
        CssUnit::Svh => "svh",
        CssUnit::Lvw => "lvw",
        CssUnit::Lvh => "lvh",
        CssUnit::Dvw => "dvw",
        CssUnit::Dvh => "dvh",
      }
    )
  }
}
