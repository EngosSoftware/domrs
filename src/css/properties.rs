use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum CssProperty {
  AlignContent,
  AlignItems,
  AlignSelf,
  BackgroundColor,
  Border,
  Bottom,
  BreakInside,
  Color,
  Display,
  FlexDirection,
  FontFamily,
  FontSize,
  FontWeight,
  GridGap,
  Height,
  JustifyContent,
  Left,
  Padding,
  Position,
  Right,
  TextAlign,
  Top,
  Width,
}

impl Display for CssProperty {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CssProperty::AlignContent => "align-content",
        CssProperty::AlignItems => "align-items",
        CssProperty::AlignSelf => "align-self",
        CssProperty::BackgroundColor => "background-color",
        CssProperty::Border => "border",
        CssProperty::Bottom => "bottom",
        CssProperty::BreakInside => "break-inside",
        CssProperty::Color => "color",
        CssProperty::Display => "display",
        CssProperty::FontFamily => "font-family",
        CssProperty::FontSize => "font-size",
        CssProperty::FontWeight => "font-weight",
        CssProperty::FlexDirection => "flex-direction",
        CssProperty::GridGap => "grid-gap",
        CssProperty::Height => "height",
        CssProperty::JustifyContent => "justify-content",
        CssProperty::Left => "left",
        CssProperty::Padding => "padding",
        CssProperty::Position => "position",
        CssProperty::Right => "right",
        CssProperty::TextAlign => "text-align",
        CssProperty::Top => "top",
        CssProperty::Width => "width",
      }
    )
  }
}
