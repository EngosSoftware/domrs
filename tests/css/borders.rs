use domrs::CssBorderStyle;

#[test]
fn display_should_work() {
  assert_eq!("dotted", CssBorderStyle::Dotted.clone().to_string());
  assert_eq!("dashed", CssBorderStyle::Dashed.clone().to_string());
  assert_eq!("solid", CssBorderStyle::Solid.clone().to_string());
  assert_eq!("double", CssBorderStyle::Double.clone().to_string());
  assert_eq!("groove", CssBorderStyle::Groove.clone().to_string());
  assert_eq!("ridge", CssBorderStyle::Ridge.clone().to_string());
  assert_eq!("inset", CssBorderStyle::Inset.clone().to_string());
  assert_eq!("outset", CssBorderStyle::Outset.clone().to_string());
  assert_eq!("none", CssBorderStyle::None.clone().to_string());
  assert_eq!("hidden", CssBorderStyle::Hidden.clone().to_string());
}
