use crate::html::DEFAULT_HTML_OFFSET;
use crate::{HtmlBodyElement, HtmlElement, HtmlHeadElement, ToText, DEFAULT_HTML_DOCTYPE, DEFAULT_HTML_INDENT, DEFAULT_HTML_LANGUAGE, DEFAULT_HTML_NAMESPACE};
use std::fmt::{Display, Write};
use std::{fmt, fs, io};

/// A structure representing HTML document.
#[derive(Debug, Clone)]
pub struct HtmlDocument {
  /// Document type.
  doctype: Option<String>,
  /// Document element.
  root: HtmlElement,
}

impl Default for HtmlDocument {
  /// Creates the default HTML document.
  fn default() -> Self {
    Self {
      doctype: None,
      root: HtmlElement::new("html").no_indent(),
    }
  }
}

impl HtmlDocument {
  /// Creates an empty HTML document.
  pub fn new() -> Self {
    Self::default()
  }

  /// Uses default doctype declaration.
  pub fn default_doctype(mut self) -> Self {
    self.doctype = DEFAULT_HTML_DOCTYPE.to_string().into();
    self
  }

  /// Uses custom doctype declaration.
  pub fn doctype(mut self, doctype: &str) -> Self {
    self.doctype = doctype.to_string().into();
    self
  }

  /// Uses default namespace.
  pub fn default_namespace(mut self) -> Self {
    self.root.set_attribute("xmlns", DEFAULT_HTML_NAMESPACE);
    self
  }

  /// Uses custom namespace.
  pub fn namespace(mut self, namespace: &str) -> Self {
    self.root.set_attribute("xmlns", namespace);
    self
  }

  /// Uses default language.
  pub fn default_language(mut self) -> Self {
    self.root.set_attribute("lang", DEFAULT_HTML_LANGUAGE);
    self
  }

  /// Uses custom language.
  pub fn language(mut self, language: &str) -> Self {
    self.root.set_attribute("lang", language);
    self
  }

  /// Uses custom head element.
  pub fn head(mut self, head: HtmlHeadElement) -> Self {
    self.root.add_child(head);
    self
  }

  /// Uses custom body element.
  pub fn body(mut self, body: HtmlBodyElement) -> Self {
    self.root.add_child(body);
    self
  }

  /// Saves the document to specified file.
  pub fn save(&self, file_name: &str, offset: usize, indent: usize) -> io::Result<()> {
    fs::write(file_name, self.to_text(offset, indent))
  }
}

impl ToText for HtmlDocument {
  /// Converts [HtmlDocument] to textual representation using provided offset and indent.
  fn to_text(&self, offset: usize, indent: usize) -> String {
    let mut buffer = String::new();
    if let Some(doctype) = &self.doctype {
      let _ = writeln!(&mut buffer, "{}", doctype);
    }
    self.root.write(offset, indent, &mut buffer);
    let _ = writeln!(&mut buffer);
    buffer
  }
}

impl Display for HtmlDocument {
  /// Converts [HtmlDocument] into its text representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_text(DEFAULT_HTML_OFFSET, DEFAULT_HTML_INDENT))
  }
}
