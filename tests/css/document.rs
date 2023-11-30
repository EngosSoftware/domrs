use domrs::{CssColor, CssDocument, CssElement, CssGroup, CssProperty, CssRuleset};

#[test]
fn display_should_work() {
  let ruleset = CssRuleset::new("p".into()).declaration(CssProperty::Color, CssColor::AliceBlue);
  let group = CssGroup::media_print().ruleset(ruleset.clone());
  let ruleset_element = CssElement::Ruleset(ruleset.clone());
  let group_element = CssElement::Group(group.clone());
  assert_eq!("", CssDocument::new().clone().to_string());
  assert_eq!("p {\n  color: aliceblue;\n}\n", CssDocument::new().ruleset(ruleset.clone()).to_string());
  assert_eq!(
    "@media print {\n  p {\n    color: aliceblue;\n  }\n}\n",
    CssDocument::new().group(group.clone()).to_string()
  );
  assert_eq!(
    "p {\n  color: aliceblue;\n}\n@media print {\n  p {\n    color: aliceblue;\n  }\n}\n",
    CssDocument::new().ruleset(ruleset).group(group).to_string()
  );
  assert_eq!("p {\n  color: aliceblue;\n}\n", CssDocument::new().element(ruleset_element.clone()).to_string());
  assert_eq!(
    "@media print {\n  p {\n    color: aliceblue;\n  }\n}\n",
    CssDocument::new().element(group_element.clone()).to_string()
  );
  assert_eq!(
    "p {\n  color: aliceblue;\n}\n@media print {\n  p {\n    color: aliceblue;\n  }\n}\n",
    CssDocument::new().element(ruleset_element).element(group_element).to_string()
  );
}

#[test]
fn adding_should_work() {
  let ruleset = CssRuleset::new("p".into()).declaration(CssProperty::Color, CssColor::AliceBlue);
  let group = CssGroup::media_print().ruleset(ruleset.clone());
  let ruleset_element = CssElement::Ruleset(ruleset);
  let group_element = CssElement::Group(group);
  assert_eq!(
    "p {\n  color: aliceblue;\n}\n@media print {\n  p {\n    color: aliceblue;\n  }\n}\n",
    (CssDocument::new().element(ruleset_element) + CssDocument::new().element(group_element)).to_string()
  );
}
