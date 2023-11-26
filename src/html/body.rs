use crate::HtmlElement;

/// Structure representing the HTML `head` element.
#[derive(Debug, Default, Clone)]
pub struct HtmlBodyElement {
  children: Vec<HtmlElement>,
}

impl HtmlBodyElement {
  pub fn child(mut self, child: impl Into<HtmlElement>) -> Self {
    self.add_child(child);
    self
  }

  pub fn add_child(&mut self, child: impl Into<HtmlElement>) {
    self.children.push(child.into());
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
