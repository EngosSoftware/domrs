use super::*;
use domrs::SvgDocument;

#[test]
fn default_should_work() {
  eq(r#"<svg/>"#, SvgDocument::default());
}

#[test]
fn new_should_work() {
  eq(r#"<svg/>"#, SvgDocument::new());
}

#[test]
fn default_namespace_should_work() {
  let expected = r#"<svg xmlns="http://www.w3.org/2000/svg"/>"#;
  let mut svg = SvgDocument::new();
  svg.set_default_namespace();
  eq(expected, svg);
  eq(expected, SvgDocument::new().default_namespace());
}

#[test]
fn custom_namespace_should_work() {
  let custom_namespace = "http://www.w3.org/2023/svg";
  let expected = r#"<svg xmlns="http://www.w3.org/2023/svg"/>"#;
  let mut svg = SvgDocument::new();
  svg.set_namespace(custom_namespace);
  eq(expected, svg);
  eq(expected, SvgDocument::new().namespace(custom_namespace));
}

#[test]
fn view_box_should_work() {
  let expected = r#"<svg viewBox="1 2 100 200"/>"#;
  let mut svg = SvgDocument::new();
  svg.set_view_box(1, 2, 100, 200);
  eq(expected, svg);
  eq(expected, SvgDocument::new().view_box(1, 2, 100, 200));
}

#[test]
fn width_and_height_should_work() {
  let expected = r#"<svg width="100" height="200"/>"#;
  let mut svg = SvgDocument::new();
  svg.set_width(100);
  svg.set_height(200);
  eq(expected, svg);
  eq(expected, SvgDocument::new().width(100).height(200));
}
