use crate::{HtmlElement, SvgElement};

#[test]
fn default_should_work() {
  let svg: HtmlElement = SvgElement::default().into();
  assert_eq!(r#"<svg xmlns="http://www.w3.org/2000/svg"/>"#, svg.to_string());
}

#[test]
fn width_and_height_should_work() {
  let svg: HtmlElement = SvgElement::default().with_width("100".into()).with_height("200".into()).into();
  assert_eq!(r#"<svg width="100" height="200" xmlns="http://www.w3.org/2000/svg"/>"#, svg.to_string());
}
