use crate::utils::get_indentation;
use crate::{HtmlAttribute, DEFAULT_HTML_INDENT};
use std::fmt;
use std::fmt::{Debug, Display, Write};

#[derive(Debug, Clone)]
pub struct HtmlElement {
  name: String,
  attributes: Vec<HtmlAttribute>,
  content: Option<String>,
  children: Vec<HtmlElement>,
  no_closing: bool,
  no_indent: bool,
}

impl Display for HtmlElement {
  /// Converts `HTML` element into text.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buffer = String::new();
    self.write(0, DEFAULT_HTML_INDENT, &mut buffer);
    write!(f, "{}", buffer)
  }
}

impl HtmlElement {
  /// Creates a new HTML element with specified name.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      attributes: vec![],
      content: None,
      children: vec![],
      no_closing: false,
      no_indent: false,
    }
  }

  pub(crate) fn no_indent(mut self) -> Self {
    self.no_indent = true;
    self
  }

  pub(crate) fn no_closing(mut self) -> Self {
    self.no_closing = true;
    self
  }

  /// Creates a new `div` element with optional class.
  pub fn new_div(class: Option<&str>) -> Self {
    let mut element = Self::new("div");
    if let Some(class_name) = class {
      element.set_class(class_name);
    }
    element
  }

  pub fn attr<T: ToString>(mut self, name: &str, value: T) -> Self {
    self.attributes.push(HtmlAttribute {
      name: name.to_string(),
      value: value.to_string(),
    });
    self
  }

  /// Sets an attribute of the HTML element.
  pub fn set_attr<T: ToString>(&mut self, name: &str, value: T) {
    self.attributes.push(HtmlAttribute {
      name: name.to_string(),
      value: value.to_string(),
    })
  }

  /// Sets a `class` attribute of the HTML element.
  pub fn set_class(&mut self, class: &str) {
    self.attributes.push(HtmlAttribute {
      name: "class".to_string(),
      value: class.to_string(),
    })
  }

  /// Sets a `style` attribute of the HTML element.
  pub fn set_style(&mut self, style: &str) {
    self.attributes.push(HtmlAttribute {
      name: "style".to_string(),
      value: style.to_string(),
    })
  }

  /// Adds a child element.
  pub fn add_child(&mut self, e: HtmlElement) {
    self.children.push(e);
  }

  /// Adds an optional child element.
  pub fn add_child_opt(&mut self, e: Option<HtmlElement>) {
    if let Some(element) = e {
      self.children.push(element);
    }
  }

  /// Adds multiple children elements.
  pub fn add_children(&mut self, elements: Vec<HtmlElement>) {
    for element in elements {
      self.children.push(element);
    }
  }

  /// Sets the content of the HTML element.
  pub fn set_content(&mut self, content: String) {
    self.content = content.into();
  }

  pub fn content(mut self, content: String) -> Self {
    self.content = content.into();
    self
  }

  /// Serializes the element to its textual representation.
  pub fn write(&self, mut offset: usize, indent: usize, buffer: &mut String) {
    if self.no_indent && offset >= indent {
      offset -= indent;
    }
    let _ = write!(buffer, "{}<{}", get_indentation(self.no_indent, offset), self.name);
    for attribute in &self.attributes {
      let _ = write!(buffer, r#" {}="{}""#, attribute.name, attribute.value);
    }
    if self.children.is_empty() {
      if let Some(content) = &self.content {
        let line_count = content.lines().count();
        if line_count > 1 {
          let _ = write!(buffer, ">");
          for line in content.lines() {
            let _ = write!(buffer, "\n{}{}", get_indentation(false, offset + indent), line);
          }
          let _ = write!(buffer, "\n{}</{}>", get_indentation(false, offset), self.name);
        } else {
          let _ = write!(buffer, ">{}</{}>", content, self.name);
        }
      } else {
        let _ = write!(buffer, "{}", if self.no_closing { ">".to_string() } else { "/>".to_string() });
      }
    } else {
      let _ = writeln!(buffer, ">");
      for (i, child) in self.children.iter().enumerate() {
        if i > 0 {
          let _ = writeln!(buffer);
        }
        child.write(offset + indent, indent, buffer);
      }
      let _ = write!(buffer, "\n{}</{}>", get_indentation(self.no_indent, offset), self.name);
    }
  }
}
