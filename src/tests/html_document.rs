use crate::{HtmlBodyElement, HtmlDocument, HtmlElement, HtmlHeadElement, HtmlLinkElement};

#[test]
fn empty_document_should_work() {
  let doc = HtmlDocument::new("en", None, None);
  assert_eq!(
    r#"<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml"/>"#,
    doc.to_string()
  );
}

#[test]
fn document_with_head_should_work() {
  let head = HtmlHeadElement::default();
  let doc = HtmlDocument::new("en", head.into(), None);
  assert_eq!(
    r#"<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml">
<head/>
</html>"#,
    doc.to_string()
  );
}

#[test]
fn document_with_body_should_work() {
  let body = HtmlBodyElement::default();
  let doc = HtmlDocument::new("en", None, body.into());
  assert_eq!(
    r#"<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml">
<body/>
</html>"#,
    doc.to_string()
  );
}

#[test]
fn document_with_head_and_body_should_work() {
  let head = HtmlHeadElement::default();
  let body = HtmlBodyElement::default();
  let doc = HtmlDocument::new("en", head.into(), body.into());
  assert_eq!(
    r#"<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml">
<head/>
<body/>
</html>"#,
    doc.to_string()
  );
}

#[test]
fn a() {
  let link = HtmlLinkElement::default().with_stylesheet("https://fonts.googleapis.com/css2?family=Asap:wght@500&family=Dosis:wght@600&display=swap");
  let head = HtmlHeadElement::default().with_charset("UTF-8").with_title("TITLE").with_link(link);
  let mut body = HtmlBodyElement::default();
  body.add_child(HtmlElement::new("div"));
  let doc = HtmlDocument::new("en", head.into(), body.into());
  assert_eq!(
    r#"<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml">
<head>
  <meta charset="UTF-8">
  <title>TITLE</title>
  <link href="https://fonts.googleapis.com/css2?family=Asap:wght@500&family=Dosis:wght@600&display=swap" rel="stylesheet">
</head>
<body>
  <div/>
</body>
</html>"#,
    doc.to_string()
  );
}
