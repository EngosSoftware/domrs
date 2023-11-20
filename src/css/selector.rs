use std::fmt;
use std::fmt::Display;

pub type CssSelectorComponent = String;

#[derive(Default, Debug, Clone)]
pub struct CssSelectorPart {
  components: Vec<CssSelectorComponent>,
}

impl CssSelectorPart {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn component(mut self, component: CssSelectorComponent) -> Self {
    self.components.push(component);
    self
  }

  pub fn class(mut self, class: &str) -> Self {
    self.components.push(format!(".{}", class));
    self
  }
}

impl Display for CssSelectorPart {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.components.to_vec().join(" "))
  }
}

#[derive(Default, Debug, Clone)]
pub struct CssSelector {
  parts: Vec<CssSelectorPart>,
}

impl CssSelector {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn part(mut self, part: CssSelectorPart) -> Self {
    self.parts.push(part);
    self
  }

  pub fn class(mut self, class: &str) -> Self {
    self.parts.push(CssSelectorPart::default().component(format!(".{}", class)));
    self
  }
}

impl Display for CssSelector {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.parts.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "))
  }
}

impl From<&str> for CssSelector {
  fn from(value: &str) -> Self {
    Self {
      parts: vec![CssSelectorPart::default().component(value.into())],
    }
  }
}

impl From<CssSelectorPart> for CssSelector {
  fn from(value: CssSelectorPart) -> Self {
    Self { parts: vec![value] }
  }
}
