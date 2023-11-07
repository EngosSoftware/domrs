use crate::{HtmlBodyElement, HtmlElement, HtmlHeadElement};
use std::fmt::{Display, Write};
use std::{fmt, fs, io};

/// Reference of the `HTML` standard.
const HREF_XMLNS: &str = "http://www.w3.org/1999/xhtml";

/// Structure representing the HTML document.
#[derive(Debug, Clone)]
pub struct HtmlDocument {
  root: HtmlElement,
}

impl HtmlDocument {
  ///
  pub fn new(lang: &str, head: Option<HtmlHeadElement>, body: Option<HtmlBodyElement>) -> Self {
    let mut root = HtmlElement::new("html").no_indent();
    root.set_attr("lang", lang);
    root.set_attr("xmlns", HREF_XMLNS);
    if let Some(head) = head {
      root.add_child(head.into());
    }
    if let Some(body) = body {
      root.add_child(body.into());
    }
    Self { root }
  }

  /// Saves the document to specified file.
  pub fn save(&self, file_name: &str) -> io::Result<()> {
    fs::write(file_name, format!("{}", self))
  }
}

impl Display for HtmlDocument {
  /// Converts `HTML` document into text.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buffer = String::new();
    let _ = writeln!(&mut buffer, "<!DOCTYPE html>");
    self.root.write(0, &mut buffer);
    write!(f, "{}", buffer)
  }
}
