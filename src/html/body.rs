use crate::HtmlElement;

/// Structure representing the HTML `head` element.
#[derive(Default, Debug, Clone)]
pub struct HtmlBodyElement {
  children: Vec<HtmlElement>,
}

impl HtmlBodyElement {
  pub fn child(mut self, child: impl Into<HtmlElement>) -> Self {
    self.children.push(child.into());
    self
  }

  pub fn add_child(&mut self, child: impl Into<HtmlElement>) {
    self.children.push(child.into());
  }

  pub fn add_br(&mut self) {
    self.children.push(HtmlElement::new("br"));
  }
}

impl From<HtmlBodyElement> for HtmlElement {
  fn from(value: HtmlBodyElement) -> Self {
    let mut body = HtmlElement::new("body").no_indent();
    for child in value.children {
      body.add_child(child);
    }
    body
  }
}
