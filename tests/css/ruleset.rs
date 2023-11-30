use domrs::{px, CssColor, CssDeclaration, CssNumber, CssProperty, CssRuleset, CssUnit, CssValue};

#[test]
fn display_should_work() {
  let ruleset = CssRuleset::new("p".into()).declaration(CssProperty::Width, CssValue::Num1(CssNumber::new(1.23, 2, CssUnit::Px)));
  assert_eq!("p {\n  width: 1.23px;\n}\n", ruleset.clone().to_string())
}

#[test]
fn adding_declarations_should_work() {
  let mut ruleset = CssRuleset::new("p".into());
  ruleset.add_declaration(CssProperty::Color, CssColor::Black);
  ruleset.add_declaration(CssProperty::Margin, px!(2));
  let expected = "p {\n  color: black;\n  margin: 2px;\n}\n";
  assert_eq!(expected, ruleset.to_string());
  assert_eq!(
    expected,
    CssRuleset::new("p".into())
      .declaration(CssProperty::Color, CssColor::Black)
      .declaration(CssProperty::Margin, px!(2))
      .to_string()
  );
}

#[test]
fn adding_multiple_declarations_should_work() {
  let mut ruleset = CssRuleset::new("p".into());
  let decl1 = CssDeclaration::new(CssProperty::Color, CssColor::Black);
  let decl2 = CssDeclaration::new(CssProperty::Margin, px!(2));
  ruleset.add_declarations(&[decl1.clone(), decl2.clone()]);
  let expected = "p {\n  color: black;\n  margin: 2px;\n}\n";
  assert_eq!(expected, ruleset.to_string());
  assert_eq!(expected, CssRuleset::new("p".into()).declarations(&[decl1, decl2]).to_string());
}
