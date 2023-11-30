use domrs::{CssColor, CssGroup, CssProperty, CssRuleset};

#[test]
fn display_should_work() {
  let ruleset = CssRuleset::new("p".into()).declaration(CssProperty::Color, CssColor::AliceBlue);
  assert_eq!(
    "@media print {\n  p {\n    color: aliceblue;\n  }\n}\n",
    CssGroup::media_print().ruleset(ruleset.clone()).clone().to_string()
  );
  assert_eq!(
    "@media screen {\n  p {\n    color: aliceblue;\n  }\n}\n",
    CssGroup::media_screen().ruleset(ruleset).to_string()
  )
}

#[test]
fn media_query_should_work() {
  let ruleset = CssRuleset::new("p".into()).declaration(CssProperty::Color, CssColor::AliceBlue);
  assert_eq!(
    "@media print screen {\n  p {\n    color: aliceblue;\n  }\n}\n",
    CssGroup::media("print screen").ruleset(ruleset).clone().to_string()
  );
}
