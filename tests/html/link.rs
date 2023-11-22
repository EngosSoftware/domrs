use domrs::{HtmlElement, HtmlLinkElement};

#[test]
fn default_should_work() {
  let link: HtmlElement = HtmlLinkElement::default().into();
  assert_eq!(r#"<link>"#, format!("{}", link));
}

#[test]
fn rel_should_work() {
  let link: HtmlElement = HtmlLinkElement::default().rel("stylesheet").into();
  assert_eq!(r#"<link rel="stylesheet">"#, format!("{}", link));
}

#[test]
fn href_should_work() {
  let link: HtmlElement = HtmlLinkElement::default().href("https://domrs.com/main.css").into();
  assert_eq!(r#"<link href="https://domrs.com/main.css">"#, format!("{}", link));
}

#[test]
fn rel_and_href_should_work() {
  let link: HtmlElement = HtmlLinkElement::default().rel("stylesheet").href("https://domrs.com/main.css").into();
  assert_eq!(r#"<link href="https://domrs.com/main.css" rel="stylesheet">"#, format!("{}", link));
}

#[test]
fn href_and_rel_should_work() {
  let link: HtmlElement = HtmlLinkElement::default().href("https://domrs.com/main.css").rel("stylesheet").into();
  assert_eq!(r#"<link href="https://domrs.com/main.css" rel="stylesheet">"#, format!("{}", link));
}
