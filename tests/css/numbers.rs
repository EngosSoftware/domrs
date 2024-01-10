use domrs::{auto, css_num, CssNumber, CssUnit};

#[test]
fn display_should_work() {
  assert_eq!("1.2346cm", CssNumber::new(1.23456, 4, CssUnit::Cm).to_string());
  assert_eq!("0", CssNumber::new(0.0, 4, CssUnit::Px).to_string());
  assert_eq!("auto", CssNumber::new(12.9876, 3, CssUnit::Auto).to_string());
}

#[test]
fn macro_auto_should_work() {
  assert_eq!("auto", auto!().to_string());
}

#[test]
fn macro_css_num_should_work() {
  assert_eq!("1.23cm", css_num!(1.23, 2, CssUnit::Cm).to_string());
}
