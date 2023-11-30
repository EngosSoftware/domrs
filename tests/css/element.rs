use domrs::{CssColor, CssElement, CssGroup, CssProperty, CssRuleset};

#[test]
fn display_should_work() {
  let ruleset = CssRuleset::new("p".into()).declaration(CssProperty::Color, CssColor::AliceBlue);
  assert_eq!("p {\n  color: aliceblue;\n}\n", CssElement::Ruleset(ruleset.clone()).clone().to_string());
  let group = CssGroup::media_print().ruleset(ruleset);
  assert_eq!("@media print {\n  p {\n    color: aliceblue;\n  }\n}\n", CssElement::Group(group).clone().to_string());
}
