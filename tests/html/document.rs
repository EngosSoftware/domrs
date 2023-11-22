use super::*;
use domrs::{HtmlBodyElement, HtmlDocument, HtmlHeadElement};

#[test]
fn html_document_default_should_work() {
  eq(D001, HtmlDocument::default());
}

#[test]
fn html_document_default_doctype_should_work() {
  eq(D002, HtmlDocument::default().default_doctype());
}

#[test]
fn html_document_custom_doctype_should_work() {
  eq(
    D003,
    HtmlDocument::default().doctype(r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#),
  );
}

#[test]
fn html_document_default_namespace_should_work() {
  eq(D004, HtmlDocument::default().default_namespace());
}

#[test]
fn html_document_custom_namespace_should_work() {
  eq(D005, HtmlDocument::default().namespace("http://www.w3.org/1998/Math/MathML"));
}

#[test]
fn html_document_default_language_should_work() {
  eq(D006, HtmlDocument::default().default_language());
}

#[test]
fn html_document_custom_language_should_work() {
  eq(D007, HtmlDocument::default().language("de"));
}

#[test]
fn html_document_empty_should_work() {
  eq(D008, HtmlDocument::default().default_doctype().default_language().default_namespace());
}

#[test]
fn html_document_with_head_should_work() {
  eq(D009, HtmlDocument::default().head(HtmlHeadElement::default()));
}

#[test]
fn html_document_with_body_should_work() {
  eq(D010, HtmlDocument::default().body(HtmlBodyElement::default()));
}

#[test]
fn html_document_with_head_and_body_should_work() {
  eq(D011, HtmlDocument::default().head(HtmlHeadElement::default()).body(HtmlBodyElement::default()));
}

#[test]
fn html_document_advanced_should_work() {
  eq(
    D012,
    HtmlDocument::default()
      .default_doctype()
      .default_language()
      .default_namespace()
      .head(HtmlHeadElement::default())
      .body(HtmlBodyElement::default()),
  );
}
