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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display_should_work() {
    assert_eq!("align-content", CssProperty::AlignContent.to_string());
    assert_eq!("align-items", CssProperty::AlignItems.to_string());
    assert_eq!("align-self", CssProperty::AlignSelf.to_string());
    assert_eq!("background-color", CssProperty::BackgroundColor.to_string());
    assert_eq!("border", CssProperty::Border.to_string());
    assert_eq!("bottom", CssProperty::Bottom.to_string());
    assert_eq!("break-inside", CssProperty::BreakInside.to_string());
    assert_eq!("color", CssProperty::Color.to_string());
    assert_eq!("display", CssProperty::Display.to_string());
    assert_eq!("font-family", CssProperty::FontFamily.to_string());
    assert_eq!("font-size", CssProperty::FontSize.to_string());
    assert_eq!("font-weight", CssProperty::FontWeight.to_string());
    assert_eq!("flex-direction", CssProperty::FlexDirection.to_string());
    assert_eq!("grid-gap", CssProperty::GridGap.to_string());
    assert_eq!("height", CssProperty::Height.to_string());
    assert_eq!("left", CssProperty::Left.to_string());
    assert_eq!("padding", CssProperty::Padding.to_string());
    assert_eq!("position", CssProperty::Position.to_string());
    assert_eq!("right", CssProperty::Right.to_string());
    assert_eq!("text-align", CssProperty::TextAlign.to_string());
    assert_eq!("top", CssProperty::Top.to_string());
    assert_eq!("width", CssProperty::Width.to_string());
  }
}
