use crate::HtmlElement;

/// Structure representing the HTML `<link>` element.
#[derive(Default, Debug, Clone)]
pub struct HtmlLinkElement {
  rel: Option<String>,
  href: Option<String>,
}

impl HtmlLinkElement {
  pub fn with_rel(mut self, rel: &str) -> Self {
    self.rel = rel.to_string().into();
    self
  }

  pub fn with_href(mut self, href: &str) -> Self {
    self.href = href.to_string().into();
    self
  }

  pub fn with_stylesheet(mut self, href: &str) -> Self {
    self.rel = "stylesheet".to_string().into();
    self.href = href.to_string().into();
    self
  }
}

impl From<HtmlLinkElement> for HtmlElement {
  fn from(value: HtmlLinkElement) -> Self {
    let mut link = HtmlElement::new("link").hide_closing_tag();
    if let Some(href) = value.href {
      link.set_attr("href", href);
    }
    if let Some(rel) = value.rel {
      link.set_attr("rel", rel);
    }
    link
  }
}
