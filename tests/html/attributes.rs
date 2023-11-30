use domrs::HtmlAttribute;

#[test]
fn debug_should_work() {
  let attribute = HtmlAttribute::new("alt", "image");
  let expected = r#"HtmlAttribute { name: "alt", value: "image" }"#;
  assert_eq!(expected, format!("{:?}", attribute));
  assert_eq!(expected, format!("{:?}", attribute.clone()));
}
