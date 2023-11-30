use super::*;
use domrs::{div, span, HtmlBodyElement, HtmlElement, HtmlHeadElement};

#[test]
fn headings_should_work() {
  eq(
    E001,
    doc().head(HtmlHeadElement::default().default_charset().title("Heading")).body(
      HtmlBodyElement::default()
        .child(HtmlElement::h1("Heading 1"))
        .child(HtmlElement::h2("Heading 2"))
        .child(HtmlElement::h3("Heading 3"))
        .child(HtmlElement::h4("Heading 4"))
        .child(HtmlElement::h5("Heading 5"))
        .child(HtmlElement::h6("Heading 6")),
    ),
  );
}

#[test]
fn div_should_work() {
  eq(
    E002,
    doc().head(HtmlHeadElement::default().default_charset().title("Div")).body(
      HtmlBodyElement::default()
        .child(HtmlElement::div().content("Before"))
        .child(HtmlElement::br())
        .child(HtmlElement::div())
        .child(HtmlElement::br())
        .child(HtmlElement::div().content("After")),
    ),
  );
}

#[test]
fn setting_class_should_work() {
  let mut div = HtmlElement::div();
  div.set_class("coloured");
  let expected = "<div class=\"coloured\"></div>";
  assert_eq!(expected, div.to_string());
  assert_eq!(expected, div!().class("coloured").to_string());
}

#[test]
fn setting_style_should_work() {
  let mut div = HtmlElement::div();
  div.set_style("text-align:right");
  let expected = "<div style=\"text-align:right\"></div>";
  assert_eq!(expected, div.to_string());
  assert_eq!(expected, div!().style("text-align:right").to_string());
}

#[test]
fn adding_optional_child_should_work() {
  let mut div = HtmlElement::div();
  let span = HtmlElement::span();
  div.add_opt_child(None);
  assert_eq!("<div></div>", div.to_string());
  assert_eq!("<div></div>", div!().opt_child(None).to_string());
  div.add_opt_child(span.into());
  assert_eq!("<div>\n  <span></span>\n</div>", div.to_string());
  assert_eq!("<div>\n  <span></span>\n</div>", div!().opt_child(span!().into()).to_string());
}

#[test]
fn adding_children_should_work() {
  let mut div = HtmlElement::div();
  let h1 = HtmlElement::h1("hello");
  let h2 = HtmlElement::h2("world");
  div.add_children(&[h1.clone(), h2.clone()]);
  let expected = "<div>\n  <h1>hello</h1>\n  <h2>world</h2>\n</div>";
  assert_eq!(expected, div.to_string());
  assert_eq!(expected, div!().children(&[h1, h2]).to_string());
}

#[test]
fn setting_attribute_should_work() {
  let mut area = HtmlElement::new("area");
  area.set_attribute("shape", "poly");
  let expected = "<area shape=\"poly\"/>";
  assert_eq!(expected, area.to_string());
  assert_eq!(expected, HtmlElement::new("area").attribute("shape", "poly").to_string());
}
