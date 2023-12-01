use super::*;
use domrs::{HtmlBodyElement, HtmlDocument, HtmlHeadElement, DEFAULT_HTML_INDENT, DEFAULT_HTML_OFFSET};
use std::fs;

#[test]
fn default_should_work() {
  eq(D001, HtmlDocument::default());
}

#[test]
fn new_should_work() {
  eq(D001, HtmlDocument::new());
}

#[test]
fn default_doctype_should_work() {
  eq(D002, HtmlDocument::new().default_doctype());
}

#[test]
fn custom_doctype_should_work() {
  eq(
    D003,
    HtmlDocument::new().doctype(r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#),
  );
}

#[test]
fn default_namespace_should_work() {
  eq(D004, HtmlDocument::new().default_namespace());
}

#[test]
fn custom_namespace_should_work() {
  eq(D005, HtmlDocument::new().namespace("http://www.w3.org/1998/Math/MathML"));
}

#[test]
fn default_language_should_work() {
  eq(D006, HtmlDocument::new().default_language());
}

#[test]
fn custom_language_should_work() {
  eq(D007, HtmlDocument::new().language("de"));
}

#[test]
fn html_document_empty_should_work() {
  eq(D008, HtmlDocument::new().default_doctype().default_language().default_namespace());
}

#[test]
fn html_document_with_head_should_work() {
  eq(D009, HtmlDocument::new().head(HtmlHeadElement::default()));
}

#[test]
fn html_document_with_body_should_work() {
  eq(D010, HtmlDocument::new().body(HtmlBodyElement::default()));
}

#[test]
fn html_document_with_head_and_body_should_work() {
  eq(D011, HtmlDocument::new().head(HtmlHeadElement::default()).body(HtmlBodyElement::default()));
}

#[test]
fn html_document_advanced_should_work() {
  eq(
    D012,
    HtmlDocument::new()
      .default_doctype()
      .default_language()
      .default_namespace()
      .head(HtmlHeadElement::default())
      .body(HtmlBodyElement::default()),
  );
}

#[test]
fn saving_html_document_to_file_should_work() {
  let doc = HtmlDocument::new()
    .default_doctype()
    .default_language()
    .default_namespace()
    .head(HtmlHeadElement::default())
    .body(HtmlBodyElement::default());
  fs::create_dir_all("./target/documents").unwrap();
  doc.save("./target/documents/D012.html", DEFAULT_HTML_OFFSET, DEFAULT_HTML_INDENT).unwrap();
  assert_eq!(D012, fs::read_to_string("./target/documents/D012.html").unwrap());
}
