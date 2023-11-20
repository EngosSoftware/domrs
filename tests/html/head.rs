use super::*;
use domrs::{HtmlHeadElement, HtmlLinkElement};

#[test]
fn default_should_work() {
  eq(H001, doc().with_head(HtmlHeadElement::default()));
}

#[test]
fn charset_should_work() {
  eq(H002, doc().with_head(HtmlHeadElement::default().with_charset("UTF-8")));
}

#[test]
fn title_should_work() {
  eq(H003, doc().with_head(HtmlHeadElement::default().with_title("TITLE")));
}

#[test]
fn link_should_work() {
  eq(
    H004,
    doc().with_head(HtmlHeadElement::default().with_link(HtmlLinkElement::default().with_rel("stylesheet").with_href("https://domrs.com/main.css"))),
  );
}
