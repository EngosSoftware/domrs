use crate::html_builder::HtmlElement;

#[test]
fn _0001() {
  let element = HtmlElement::new("html");
  assert_eq!(r#"<html></html>"#, format!("{}", element));
}

#[test]
fn _0002() {
  let mut element = HtmlElement::new("html");
  element.set_attr("lang", "en");
  assert_eq!(r#"<html lang="en"></html>"#, format!("{}", element));
}

#[test]
fn _0003() {
  let mut element = HtmlElement::new("html");
  element.set_attr("lang", "en");
  element.set_attr("xmlns", "http://www.w3.org/1999/html");
  assert_eq!(r#"<html lang="en" xmlns="http://www.w3.org/1999/html"></html>"#, format!("{}", element));
}

#[test]
fn _0004() {
  let mut element = HtmlElement::new("html");
  let head = HtmlElement::new("head");
  element.add_child(head);
  assert_eq!(
    r#"<html>
  <head></head>
</html>"#,
    format!("{}", element)
  );
}

#[test]
fn _0005() {
  // prepare expected output
  let expected = r#"
<html lang="en" xmlns="http://www.w3.org/1999/html">
  <head>
    <meta charset="UTF-8">
    <title>DMN Model</title>
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500&display=swap">
    <style>
      body {
        font-family: 'Roboto', sans-serif;
      }
      @media print {
        @page { margin: 0; }
        body { margin: 1.6cm; }
      }
    </style>
  </head>
  <body></body>
</html>
  "#
  .trim();
  // prepare styles
  let style_content = r#"
body {
  font-family: 'Roboto', sans-serif;
}
@media print {
  @page { margin: 0; }
  body { margin: 1.6cm; }
}
"#
  .trim();
  // prepare HTML document root
  let mut root = HtmlElement::new("html");
  root.set_attr("lang", "en");
  root.set_attr("xmlns", "http://www.w3.org/1999/html");
  // prepare HTML header
  let mut head = HtmlElement::new("head");
  // <meta>
  let mut meta = HtmlElement::new_void("meta");
  meta.set_attr("charset", "UTF-8");
  head.add_child(meta);
  // <title>
  let mut title = HtmlElement::new("title");
  title.set_content("DMN Model");
  head.add_child(title);
  // <link>
  let mut link = HtmlElement::new_void("link");
  link.set_attr("rel", "stylesheet");
  link.set_attr("href", "https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500&display=swap");
  head.add_child(link);
  // <style>
  let mut style = HtmlElement::new("style");
  style.set_content(style_content);
  head.add_child(style);
  // finalize header
  root.add_child(head);
  // prepare HTML body
  let body = HtmlElement::new("body");
  root.add_child(body);
  // compare expected output with html document
  assert_eq!(expected, format!("{}", root));
}
