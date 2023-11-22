use super::*;
use domrs::{HtmlBodyElement, HtmlElement, HtmlHeadElement};

#[test]
fn headings_should_work() {
  eq(
    E001,
    doc().head(HtmlHeadElement::default().default_charset().title("Heading")).body(
      HtmlBodyElement::default()
        .child(HtmlElement::h1("Heading 1".to_string()))
        .child(HtmlElement::h2("Heading 2".to_string()))
        .child(HtmlElement::h3("Heading 3".to_string()))
        .child(HtmlElement::h4("Heading 4".to_string()))
        .child(HtmlElement::h5("Heading 5".to_string()))
        .child(HtmlElement::h6("Heading 6".to_string())),
    ),
  );
}

#[test]
fn div_should_work() {
  eq(
    E002,
    doc().head(HtmlHeadElement::default().default_charset().title("Div")).body(
      HtmlBodyElement::default()
        .child(HtmlElement::div().content("Before".to_string()))
        .child(HtmlElement::br())
        .child(HtmlElement::div())
        .child(HtmlElement::br())
        .child(HtmlElement::div().content("After".to_string())),
    ),
  );
}
