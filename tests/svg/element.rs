use domrs::{HtmlElement, SvgDocument};

#[test]
fn default_should_work() {
  let svg: HtmlElement = SvgDocument::default().into();
  assert_eq!(r#"<svg xmlns="http://www.w3.org/2000/svg"/>"#, svg.to_string());
}

#[test]
fn width_and_height_should_work() {
  let svg: HtmlElement = SvgDocument::default().width("100".into()).height("200".into()).into();
  assert_eq!(r#"<svg width="100" height="200" xmlns="http://www.w3.org/2000/svg"/>"#, svg.to_string());
}
