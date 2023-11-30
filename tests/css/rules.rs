use domrs::CssAtRule;

#[test]
fn display_should_work() {
  assert_eq!("@media", CssAtRule::Media.clone().to_string())
}
