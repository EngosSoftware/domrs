use crate::html::HtmlStyleElement;
use crate::{HtmlElement, HtmlLinkElement};

/// Structure representing the HTML `head` element.
#[derive(Debug, Default, Clone)]
pub struct HtmlHeadElement {
  charset: Option<String>,
  title: Option<String>,
  links: Vec<HtmlLinkElement>,
  style: Option<HtmlStyleElement>,
}

impl HtmlHeadElement {
  /// Use default charset which is `UTF-8`.
  pub fn default_charset(mut self) -> Self {
    self.charset = "UTF-8".to_string().into();
    self
  }

  /// Use specified charset.
  pub fn charset(mut self, charset: &str) -> Self {
    self.charset = charset.to_string().into();
    self
  }

  /// Use specified title.
  pub fn title(mut self, title: &str) -> Self {
    self.title = title.to_string().into();
    self
  }

  /// Use specified link.
  pub fn link(mut self, link: HtmlLinkElement) -> Self {
    self.links.push(link);
    self
  }

  /// Use specified stylesheet
  pub fn stylesheet(mut self, href: &str) -> Self {
    self.links.push(HtmlLinkElement::default().stylesheet(href));
    self
  }

  /// Use specified styling.
  pub fn style(mut self, style: HtmlStyleElement) -> Self {
    self.style = style.into();
    self
  }
}

impl From<HtmlHeadElement> for HtmlElement {
  fn from(value: HtmlHeadElement) -> Self {
    let mut head = HtmlElement::new("head").no_indent();
    if let Some(charset) = value.charset {
      let mut meta = HtmlElement::new("meta").hide_closing_tag();
      meta.set_attr("charset", charset);
      head.add_child(meta);
    }
    if let Some(title) = value.title {
      let mut title_element = HtmlElement::new("title");
      title_element.set_content(title);
      head.add_child(title_element);
    }
    for link in value.links {
      head.add_child(link);
    }
    if let Some(style) = value.style {
      head.add_child(style);
    }
    head
  }
}
