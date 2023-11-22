use domrs::CssProperty;

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
  assert_eq!("justify-content", CssProperty::JustifyContent.to_string());
  assert_eq!("left", CssProperty::Left.to_string());
  assert_eq!("padding", CssProperty::Padding.to_string());
  assert_eq!("position", CssProperty::Position.to_string());
  assert_eq!("right", CssProperty::Right.to_string());
  assert_eq!("text-align", CssProperty::TextAlign.to_string());
  assert_eq!("top", CssProperty::Top.to_string());
  assert_eq!("width", CssProperty::Width.to_string());
}
