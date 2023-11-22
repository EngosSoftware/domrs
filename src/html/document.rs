use crate::html::DEFAULT_HTML_OFFSET;
use crate::{HtmlBodyElement, HtmlElement, HtmlHeadElement, DEFAULT_HTML_DOCTYPE, DEFAULT_HTML_INDENT, DEFAULT_HTML_LANGUAGE, DEFAULT_HTML_NAMESPACE};
use std::fmt::{Display, Write};
use std::{fmt, fs, io};

/// Structure representing the `HTML` document.
#[derive(Clone)]
pub struct HtmlDocument {
  /// Document type.
  doctype: Option<String>,
  /// Document element.
  root: HtmlElement,
}

impl Default for HtmlDocument {
  /// Creates the default `HTML` document.
  fn default() -> Self {
    Self {
      doctype: None,
      root: HtmlElement::new("html").no_indent(),
    }
  }
}

impl HtmlDocument {
  /// Uses default doctype declaration.
  pub fn with_default_doctype(mut self) -> Self {
    self.doctype = DEFAULT_HTML_DOCTYPE.to_string().into();
    self
  }

  /// Uses custom doctype declaration.
  pub fn with_doctype(mut self, doctype: &str) -> Self {
    self.doctype = doctype.to_string().into();
    self
  }

  /// Uses default namespace.
  pub fn with_default_namespace(mut self) -> Self {
    self.root.set_attr("xmlns", DEFAULT_HTML_NAMESPACE);
    self
  }

  /// Uses custom namespace.
  pub fn with_namespace(mut self, namespace: &str) -> Self {
    self.root.set_attr("xmlns", namespace);
    self
  }

  /// Uses default language.
  pub fn with_default_language(mut self) -> Self {
    self.root.set_attr("lang", DEFAULT_HTML_LANGUAGE);
    self
  }

  /// Uses custom language.
  pub fn with_language(mut self, language: &str) -> Self {
    self.root.set_attr("lang", language);
    self
  }

  /// Uses custom head element.
  pub fn with_head(mut self, head: HtmlHeadElement) -> Self {
    self.root.add_child(head);
    self
  }

  /// Uses custom body element.
  pub fn with_body(mut self, body: HtmlBodyElement) -> Self {
    self.root.add_child(body);
    self
  }

  /// Returns the markup text for this `HTML` document.
  pub fn to_markup(&self, offset: usize, indent: usize) -> String {
    let mut markup = String::new();
    if let Some(doctype) = &self.doctype {
      let _ = writeln!(&mut markup, "{}", doctype);
    }
    self.root.write(offset, indent, &mut markup);
    let _ = writeln!(&mut markup);
    markup
  }

  /// Saves the document to specified file.
  pub fn save(&self, file_name: &str, offset: usize, indent: usize) -> io::Result<()> {
    fs::write(file_name, self.to_markup(offset, indent))
  }
}

impl Display for HtmlDocument {
  /// Converts [HtmlDocument] into its text representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_markup(DEFAULT_HTML_OFFSET, DEFAULT_HTML_INDENT))
  }
}
