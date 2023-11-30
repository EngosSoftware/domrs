use domrs::{CssSelector, CssSelectorPart};

mod css_selector_part_tests {
  use super::*;

  #[test]
  fn new_should_work() {
    assert_eq!("", CssSelectorPart::new().to_string());
  }

  #[test]
  fn adding_class_should_work() {
    let mut part = CssSelectorPart::new();
    part.add_class("coloured");
    assert_eq!(".coloured", part.to_string());
    assert_eq!(".coloured", CssSelectorPart::new().class("coloured").to_string());
  }

  #[test]
  fn adding_multiple_classes_should_work() {
    assert_eq!(".coloured .number", CssSelectorPart::new().class("coloured").class("number").to_string());
  }

  #[test]
  fn adding_element_should_work() {
    let mut part = CssSelectorPart::new();
    part.add_element("p");
    assert_eq!("p", part.to_string());
    assert_eq!("p", CssSelectorPart::new().element("p").to_string());
  }

  #[test]
  fn adding_multiple_elements_should_work() {
    assert_eq!("p div", CssSelectorPart::new().element("p").element("div").to_string());
  }

  #[test]
  fn adding_mix_should_work() {
    assert_eq!(
      ".coloured p .number div",
      CssSelectorPart::new().class("coloured").element("p").class("number").element("div").to_string()
    );
  }

  #[test]
  fn conversions_should_work() {
    let part: CssSelectorPart = "p".into();
    assert_eq!("p", part.to_string());
    let part: CssSelectorPart = ["pre", "p", "div"].as_slice().into();
    assert_eq!("pre p div", part.to_string());
  }
}

mod css_selector_tests {
  use super::*;

  #[test]
  fn new_should_work() {
    assert_eq!("", CssSelector::new().to_string());
  }

  #[test]
  fn adding_part_should_work() {
    let part: CssSelectorPart = "p".into();
    let mut selector = CssSelector::new();
    selector.add_part(part.clone());
    assert_eq!("p", selector.to_string());
    assert_eq!("p", CssSelector::new().part(part).to_string());
  }

  #[test]
  fn adding_class_should_work() {
    let mut selector = CssSelector::new();
    selector.add_class("coloured");
    assert_eq!(".coloured", selector.to_string());
    assert_eq!(".coloured", CssSelector::new().class("coloured").to_string());
  }

  #[test]
  fn adding_multiple_classes_should_work() {
    let mut selector = CssSelector::new();
    selector.add_class("coloured");
    selector.add_class("number");
    assert_eq!(".coloured, .number", selector.to_string());
    assert_eq!(".coloured, .number", CssSelector::new().class("coloured").class("number").to_string());
  }

  #[test]
  fn adding_element_should_work() {
    let mut selector = CssSelector::new();
    selector.add_element("p");
    assert_eq!("p", selector.to_string());
    assert_eq!("p", CssSelector::new().element("p").to_string());
  }

  #[test]
  fn adding_multiple_elements_should_work() {
    let mut selector = CssSelector::new();
    selector.add_element("p");
    selector.add_element("div");
    assert_eq!("p, div", selector.to_string());
    assert_eq!("p, div", CssSelector::new().element("p").element("div").to_string());
  }

  #[test]
  fn adding_mix_should_work() {
    let mut selector = CssSelector::new();
    selector.add_element("p");
    selector.add_class("coloured");
    selector.add_element("div");
    selector.add_class("number");
    assert_eq!("p, .coloured, div, .number", selector.to_string());
    assert_eq!(
      "p, .coloured, div, .number",
      CssSelector::new().element("p").class("coloured").element("div").class("number").to_string()
    );
  }

  #[test]
  fn conversions_should_work() {
    let selector: CssSelector = "p".into();
    assert_eq!("p", selector.clone().to_string());
    let selector: CssSelector = (["pre", "p", "div"].as_slice()).into();
    assert_eq!("pre, p, div", selector.to_string());
    let selector: CssSelector = CssSelectorPart::new().element("p").into();
    assert_eq!("p", selector.to_string());
    let part1 = CssSelectorPart::new().element("p").element("div").class("coloured");
    let part2 = CssSelectorPart::new().element("pre").class("code").element("p");
    let selector: CssSelector = [part1, part2].as_slice().into();
    assert_eq!("p div .coloured, pre .code p", selector.to_string());
  }
}
