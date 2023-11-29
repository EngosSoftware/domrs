use super::*;
use domrs::{HtmlBodyElement, HtmlElement, HtmlHeadElement};

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
