use domrs::{HtmlElement, HtmlHeadElement, HtmlLinkElement};

#[test]
fn default_should_work() {
  let head: HtmlElement = HtmlHeadElement::default().into();
  assert_eq!(r#"<head/>"#, format!("{}", head));
}

#[test]
fn charset_should_work() {
  let head: HtmlElement = HtmlHeadElement::default().with_charset("UTF-8").into();
  assert_eq!(
    r#"<head>
  <meta charset="UTF-8">
</head>"#,
    format!("{}", head)
  );
}

#[test]
fn title_should_work() {
  let head: HtmlElement = HtmlHeadElement::default().with_title("TITLE").into();
  assert_eq!(
    r#"<head>
  <title>TITLE</title>
</head>"#,
    format!("{}", head)
  );
}

#[test]
fn link_should_work() {
  let link = HtmlLinkElement::default().with_rel("stylesheet").with_href("https://domrs.com/main.css");
  let head: HtmlElement = HtmlHeadElement::default().with_link(link).into();
  assert_eq!(
    r#"<head>
  <link href="https://domrs.com/main.css" rel="stylesheet">
</head>"#,
    format!("{}", head)
  );
}
