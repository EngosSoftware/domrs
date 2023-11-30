use domrs::{CssProperty, SvgAttribute};
use std::cmp::Ordering;

/// Utility function for checking display trait.
fn eq(expected: &str, attribute: SvgAttribute) {
  assert_eq!(expected, attribute.to_string());
  assert_eq!(expected, CssProperty::SvgAttribute(attribute).to_string());
}

#[test]
fn display_should_work() {
  eq("fill", SvgAttribute::Fill);
  eq("stroke", SvgAttribute::Stroke);
  eq("stroke-width", SvgAttribute::StrokeWidth);
}

#[test]
fn comparing_should_work() {
  assert!((SvgAttribute::Fill == SvgAttribute::Fill));
  assert!((SvgAttribute::Fill != SvgAttribute::Stroke));
  assert!((SvgAttribute::Fill < SvgAttribute::Stroke));
  assert_eq!(Ordering::Less, SvgAttribute::Fill.cmp(&SvgAttribute::Stroke));
  assert_eq!(Ordering::Equal, SvgAttribute::Fill.cmp(&SvgAttribute::Fill));
}
