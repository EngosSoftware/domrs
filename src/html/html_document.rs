use crate::{HtmlBodyElement, HtmlElement, HtmlHeadElement};
use std::fmt::{Display, Write};
use std::{fmt, fs, io};

/// Default `XHTML` namespace.
pub const DEFAULT_HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// Default `DOCTYPE`.
pub const DEFAULT_HTML_DOCTYPE: &str = "<!DOCTYPE html>";

/// Default language.
pub const DEFAULT_HTML_LANGUAGE: &str = "en";

/// Default indentation width.
pub const DEFAULT_HTML_INDENT: usize = 2;

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
    self.root.add_child(head.into());
    self
  }

  /// Uses custom body element.
  pub fn with_body(mut self, body: HtmlBodyElement) -> Self {
    self.root.add_child(body.into());
    self
  }

  /// Returns the markup text for this `HTML` document.
  pub fn to_markup(&self, indent: usize) -> String {
    let mut markup = String::new();
    if let Some(doctype) = &self.doctype {
      let _ = writeln!(&mut markup, "{}", doctype);
    }
    self.root.write(0, indent, &mut markup);
    markup
  }

  /// Saves the document to specified file.
  pub fn save(&self, file_name: &str, indent: usize) -> io::Result<()> {
    fs::write(file_name, self.to_markup(indent))
  }
}

impl Display for HtmlDocument {
  /// Converts [HtmlDocument] into its text representation.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_markup(DEFAULT_HTML_INDENT))
  }
}
