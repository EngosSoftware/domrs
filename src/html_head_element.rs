use crate::{HtmlElement, HtmlLinkElement};

/// Structure representing the HTML `head` element.
#[derive(Default, Debug, Clone)]
pub struct HtmlHeadElement {
  charset: Option<String>,
  title: Option<String>,
  links: Vec<HtmlLinkElement>,
}

impl HtmlHeadElement {
  /// Add charset.
  pub fn with_charset(mut self, charset: &str) -> Self {
    self.charset = charset.to_string().into();
    self
  }

  /// Add title.
  pub fn with_title(mut self, title: &str) -> Self {
    self.title = title.to_string().into();
    self
  }

  /// Add link.
  pub fn with_link(mut self, link: HtmlLinkElement) -> Self {
    self.links.push(link);
    self
  }
}

impl From<HtmlHeadElement> for HtmlElement {
  fn from(value: HtmlHeadElement) -> Self {
    let mut head = HtmlElement::new("head").no_indent();
    if let Some(charset) = value.charset {
      let mut meta = HtmlElement::new("meta").no_closing();
      meta.set_attr("charset", charset);
      head.add_child(meta);
    }
    if let Some(title) = value.title {
      let mut title_element = HtmlElement::new("title");
      title_element.set_content(&title);
      head.add_child(title_element);
    }
    for link in value.links {
      head.add_child(link.into());
    }
    head
  }
}
