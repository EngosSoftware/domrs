use domrs::{HtmlBodyElement, HtmlDocument, HtmlHeadElement};

#[test]
fn html_document_default_should_work() {
  let doc = HtmlDocument::default();
  assert_eq!(r#"<html/>"#, doc.to_string());
}

#[test]
fn html_document_default_doctype_should_work() {
  let doc = HtmlDocument::default().with_default_doctype();
  assert_eq!(
    r#"<!DOCTYPE html>
<html/>"#,
    doc.to_string()
  );
}

#[test]
fn html_document_custom_doctype_should_work() {
  let doc = HtmlDocument::default().with_doctype(r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#);
  assert_eq!(
    r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
<html/>"#,
    doc.to_string()
  );
}

#[test]
fn html_document_default_namespace_should_work() {
  let doc = HtmlDocument::default().with_default_namespace();
  assert_eq!(r#"<html xmlns="http://www.w3.org/1999/xhtml"/>"#, doc.to_string());
}

#[test]
fn html_document_custom_namespace_should_work() {
  let doc = HtmlDocument::default().with_namespace("http://www.w3.org/1998/Math/MathML");
  assert_eq!(r#"<html xmlns="http://www.w3.org/1998/Math/MathML"/>"#, doc.to_string());
}

#[test]
fn html_document_default_language_should_work() {
  let doc = HtmlDocument::default().with_default_language();
  assert_eq!(r#"<html lang="en"/>"#, doc.to_string());
}

#[test]
fn html_document_custom_language_should_work() {
  let doc = HtmlDocument::default().with_language("de");
  assert_eq!(r#"<html lang="de"/>"#, doc.to_string());
}

#[test]
fn html_document_empty_should_work() {
  let doc = HtmlDocument::default().with_default_doctype().with_default_language().with_default_namespace();
  assert_eq!(
    r#"<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml"/>"#,
    doc.to_string()
  );
}

#[test]
fn html_document_with_head_should_work() {
  let doc = HtmlDocument::default().with_head(HtmlHeadElement::default());
  assert_eq!(
    r#"<html>
<head/>
</html>"#,
    doc.to_string()
  );
}

#[test]
fn html_document_with_body_should_work() {
  let doc = HtmlDocument::default().with_body(HtmlBodyElement::default());
  assert_eq!(
    r#"<html>
<body/>
</html>"#,
    doc.to_string()
  );
}

#[test]
fn html_document_with_head_and_body_should_work() {
  let doc = HtmlDocument::default().with_head(HtmlHeadElement::default()).with_body(HtmlBodyElement::default());
  assert_eq!(
    r#"<html>
<head/>
<body/>
</html>"#,
    doc.to_string()
  );
}

#[test]
fn html_document_advanced_should_work() {
  let doc = HtmlDocument::default()
    .with_default_doctype()
    .with_default_language()
    .with_default_namespace()
    .with_head(HtmlHeadElement::default())
    .with_body(HtmlBodyElement::default());
  assert_eq!(
    r#"<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml">
<head/>
<body/>
</html>"#,
    doc.to_string()
  );
}
